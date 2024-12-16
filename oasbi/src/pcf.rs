#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	pub use crate::common::common_models::*;
	/// Error types.
	pub use crate::common::*;

	/// Contains the access network charging identifier for the PCC rule(s) or
	/// for the whole PDU session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the access network charging identifier for the
	/// PCC rule(s) or for the whole PDU session.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "accNetChaIdValue"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "accNetChargId"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "accNetChaIdValue": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "accNetChargId": {
	///      "description": "A character string containing the access network
	/// charging id.",
	///      "type": "string"
	///    },
	///    "refPccRuleIds": {
	///      "description": "Contains the identifier of the PCC rule(s)
	/// associated to the provided Access Network Charging Identifier.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sessionChScope": {
	///      "description": "When it is included and set to true, indicates the Access Network Charging Identifier applies to the whole PDU Session",
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
	pub enum AccNetChId {
		#[default]
		Variant0 {
			#[serde(rename = "accNetChaIdValue")]
			acc_net_cha_id_value: ChargingId,
			/// Contains the identifier of the PCC rule(s) associated to the
			/// provided Access Network Charging Identifier.
			#[serde(
				rename = "refPccRuleIds",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			ref_pcc_rule_ids: Vec<String>,
			/// When it is included and set to true, indicates the Access
			/// Network Charging Identifier applies to the whole PDU Session
			#[serde(
				rename = "sessionChScope",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			session_ch_scope: Option<bool>,
		},
		Variant1 {
			/// A character string containing the access network charging id.
			#[serde(rename = "accNetChargId")]
			acc_net_charg_id: String,
			/// Contains the identifier of the PCC rule(s) associated to the
			/// provided Access Network Charging Identifier.
			#[serde(
				rename = "refPccRuleIds",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			ref_pcc_rule_ids: Vec<String>,
			/// When it is included and set to true, indicates the Access
			/// Network Charging Identifier applies to the whole PDU Session
			#[serde(
				rename = "sessionChScope",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			session_ch_scope: Option<bool>,
		},
	}

	impl From<&AccNetChId> for AccNetChId {
		fn from(value: &AccNetChId) -> Self {
			value.clone()
		}
	}

	/// Describes the network entity within the access network performing
	/// charging
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the network entity within the access network
	/// performing charging",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "anChargIpv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "anChargIpv6Addr"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "anChargIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "anChargIpv6Addr": {
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
	pub enum AccNetChargingAddress {
		#[default]
		Variant0 {
			#[serde(rename = "anChargIpv4Addr")]
			an_charg_ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "anChargIpv6Addr")]
			an_charg_ipv6_addr: Ipv6Addr,
		},
	}

	impl From<&AccNetChargingAddress> for AccNetChargingAddress {
		fn from(value: &AccNetChargingAddress) -> Self {
			value.clone()
		}
	}

	/// Contains the MBS Service Information that can be accepted by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the MBS Service Information that can be
	/// accepted by the PCF.\n",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "accMbsServInfo"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "accMaxMbsBw"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "accMaxMbsBw": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "accMbsServInfo": {
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsMediaComp"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AcceptableMbsServInfo {
		#[default]
		Variant0 {
			#[serde(rename = "accMbsServInfo")]
			acc_mbs_serv_info: ::std::collections::HashMap<String, MbsMediaComp>,
		},
		Variant1 {
			#[serde(rename = "accMaxMbsBw")]
			acc_max_mbs_bw: BitRate,
		},
	}

	impl From<&AcceptableMbsServInfo> for AcceptableMbsServInfo {
		fn from(value: &AcceptableMbsServInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the maximum bandwidth that shall be authorized by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the maximum bandwidth that shall be
	/// authorized by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "accBwMedComps": {
	///      "description": "Indicates the maximum bandwidth that shall be
	/// authorized by the PCF for each media component of the map. The key of
	/// the map is the media component number.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaComponent"
	///      }
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AcceptableServiceInfo {
		/// Indicates the maximum bandwidth that shall be authorized by the PCF
		/// for each media component of the map. The key of the map is the media
		/// component number.
		#[serde(
			rename = "accBwMedComps",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub acc_bw_med_comps: ::std::collections::HashMap<String, MediaComponent>,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRate>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRate>,
	}

	impl From<&AcceptableServiceInfo> for AcceptableServiceInfo {
		fn from(value: &AcceptableServiceInfo) -> Self {
			value.clone()
		}
	}

	/// Describes the access network charging identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the access network charging identifier.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "accNetChaIdValue"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "accNetChargIdString"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "accNetChaIdValue": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "accNetChargIdString": {
	///      "description": "A character string containing the access network
	/// charging identifier.",
	///      "type": "string"
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
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
	pub enum AccessNetChargingIdentifier {
		#[default]
		Variant0 {
			#[serde(rename = "accNetChaIdValue")]
			acc_net_cha_id_value: ChargingId,
			#[serde(default, skip_serializing_if = "Vec::is_empty")]
			flows: Vec<Flows>,
		},
		Variant1 {
			/// A character string containing the access network charging
			/// identifier.
			#[serde(rename = "accNetChargIdString")]
			acc_net_charg_id_string: String,
			#[serde(default, skip_serializing_if = "Vec::is_empty")]
			flows: Vec<Flows>,
		},
	}

	impl From<&AccessNetChargingIdentifier> for AccessNetChargingIdentifier {
		fn from(value: &AccessNetChargingIdentifier) -> Self {
			value.clone()
		}
	}

	/// Contains the accumulated usage report information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the accumulated usage report information.",
	///  "type": "object",
	///  "required": [
	///    "refUmIds"
	///  ],
	///  "properties": {
	///    "nextTimeUsage": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "nextVolUsage": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "nextVolUsageDownlink": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "nextVolUsageUplink": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "refUmIds": {
	///      "description": "An id referencing UsageMonitoringData objects
	/// associated with this usage report.",
	///      "type": "string"
	///    },
	///    "timeUsage": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "volUsage": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "volUsageDownlink": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "volUsageUplink": {
	///      "$ref": "#/components/schemas/Volume"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccuUsageReport {
		#[serde(
			rename = "nextTimeUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_time_usage: Option<DurationSec>,
		#[serde(
			rename = "nextVolUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_usage: Option<Volume>,
		#[serde(
			rename = "nextVolUsageDownlink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_usage_downlink: Option<Volume>,
		#[serde(
			rename = "nextVolUsageUplink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_usage_uplink: Option<Volume>,
		/// An id referencing UsageMonitoringData objects associated with this
		/// usage report.
		#[serde(rename = "refUmIds")]
		pub ref_um_ids: String,
		#[serde(rename = "timeUsage", default, skip_serializing_if = "Option::is_none")]
		pub time_usage: Option<DurationSec>,
		#[serde(rename = "volUsage", default, skip_serializing_if = "Option::is_none")]
		pub vol_usage: Option<Volume>,
		#[serde(
			rename = "volUsageDownlink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vol_usage_downlink: Option<Volume>,
		#[serde(
			rename = "volUsageUplink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vol_usage_uplink: Option<Volume>,
	}

	impl From<&AccuUsageReport> for AccuUsageReport {
		fn from(value: &AccuUsageReport) -> Self {
			value.clone()
		}
	}

	/// Represents an accumulated usage.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an accumulated usage.",
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "duration": {
	///      "$ref": "#/components/schemas/schemas-DurationSec"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Volume"
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
	pub struct AccumulatedUsage {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Volume>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub duration: Option<SchemasDurationSec>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Volume>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Volume>,
	}

	impl From<&AccumulatedUsage> for AccumulatedUsage {
		fn from(value: &AccumulatedUsage) -> Self {
			value.clone()
		}
	}

	/// Indicates the combination of additional Access Type and RAT Type for a
	/// MA PDU session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the combination of additional Access Type and
	/// RAT Type for a MA PDU session.",
	///  "type": "object",
	///  "required": [
	///    "accessType"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AdditionalAccessInfo {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
	}

	impl From<&AdditionalAccessInfo> for AdditionalAccessInfo {
		fn from(value: &AdditionalAccessInfo) -> Self {
			value.clone()
		}
	}

	/// Represents an event to notify to the AF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an event to notify to the AF.",
	///  "type": "string",
	///  "enum": [
	///    "ACCESS_TYPE_CHANGE",
	///    "ANI_REPORT",
	///    "APP_DETECTION",
	///    "CHARGING_CORRELATION",
	///    "EPS_FALLBACK",
	///    "FAILED_QOS_UPDATE",
	///    "FAILED_RESOURCES_ALLOCATION",
	///    "OUT_OF_CREDIT",
	///    "PDU_SESSION_STATUS",
	///    "PLMN_CHG",
	///    "QOS_MONITORING",
	///    "QOS_NOTIF",
	///    "RAN_NAS_CAUSE",
	///    "REALLOCATION_OF_CREDIT",
	///    "SAT_CATEGORY_CHG",
	///    "SUCCESSFUL_QOS_UPDATE",
	///    "SUCCESSFUL_RESOURCES_ALLOCATION",
	///    "TSN_BRIDGE_INFO",
	///    "UP_PATH_CHG_FAILURE",
	///    "USAGE_REPORT"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
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
		#[serde(rename = "ACCESS_TYPE_CHANGE")]
		AccessTypeChange,
		#[serde(rename = "ANI_REPORT")]
		AniReport,
		#[serde(rename = "APP_DETECTION")]
		AppDetection,
		#[serde(rename = "CHARGING_CORRELATION")]
		ChargingCorrelation,
		#[serde(rename = "EPS_FALLBACK")]
		EpsFallback,
		#[serde(rename = "FAILED_QOS_UPDATE")]
		FailedQosUpdate,
		#[serde(rename = "FAILED_RESOURCES_ALLOCATION")]
		FailedResourcesAllocation,
		#[serde(rename = "OUT_OF_CREDIT")]
		OutOfCredit,
		#[serde(rename = "PDU_SESSION_STATUS")]
		PduSessionStatus,
		#[serde(rename = "PLMN_CHG")]
		PlmnChg,
		#[serde(rename = "QOS_MONITORING")]
		QosMonitoring,
		#[serde(rename = "QOS_NOTIF")]
		QosNotif,
		#[serde(rename = "RAN_NAS_CAUSE")]
		RanNasCause,
		#[serde(rename = "REALLOCATION_OF_CREDIT")]
		ReallocationOfCredit,
		#[serde(rename = "SAT_CATEGORY_CHG")]
		SatCategoryChg,
		#[serde(rename = "SUCCESSFUL_QOS_UPDATE")]
		SuccessfulQosUpdate,
		#[serde(rename = "SUCCESSFUL_RESOURCES_ALLOCATION")]
		SuccessfulResourcesAllocation,
		#[serde(rename = "TSN_BRIDGE_INFO")]
		TsnBridgeInfo,
		#[serde(rename = "UP_PATH_CHG_FAILURE")]
		UpPathChgFailure,
		#[serde(rename = "USAGE_REPORT")]
		UsageReport,
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
				Self::AccessTypeChange => "ACCESS_TYPE_CHANGE".to_string(),
				Self::AniReport => "ANI_REPORT".to_string(),
				Self::AppDetection => "APP_DETECTION".to_string(),
				Self::ChargingCorrelation => "CHARGING_CORRELATION".to_string(),
				Self::EpsFallback => "EPS_FALLBACK".to_string(),
				Self::FailedQosUpdate => "FAILED_QOS_UPDATE".to_string(),
				Self::FailedResourcesAllocation => "FAILED_RESOURCES_ALLOCATION".to_string(),
				Self::OutOfCredit => "OUT_OF_CREDIT".to_string(),
				Self::PduSessionStatus => "PDU_SESSION_STATUS".to_string(),
				Self::PlmnChg => "PLMN_CHG".to_string(),
				Self::QosMonitoring => "QOS_MONITORING".to_string(),
				Self::QosNotif => "QOS_NOTIF".to_string(),
				Self::RanNasCause => "RAN_NAS_CAUSE".to_string(),
				Self::ReallocationOfCredit => "REALLOCATION_OF_CREDIT".to_string(),
				Self::SatCategoryChg => "SAT_CATEGORY_CHG".to_string(),
				Self::SuccessfulQosUpdate => "SUCCESSFUL_QOS_UPDATE".to_string(),
				Self::SuccessfulResourcesAllocation => {
					"SUCCESSFUL_RESOURCES_ALLOCATION".to_string()
				}
				Self::TsnBridgeInfo => "TSN_BRIDGE_INFO".to_string(),
				Self::UpPathChgFailure => "UP_PATH_CHG_FAILURE".to_string(),
				Self::UsageReport => "USAGE_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AfEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACCESS_TYPE_CHANGE" => Ok(Self::AccessTypeChange),
				"ANI_REPORT" => Ok(Self::AniReport),
				"APP_DETECTION" => Ok(Self::AppDetection),
				"CHARGING_CORRELATION" => Ok(Self::ChargingCorrelation),
				"EPS_FALLBACK" => Ok(Self::EpsFallback),
				"FAILED_QOS_UPDATE" => Ok(Self::FailedQosUpdate),
				"FAILED_RESOURCES_ALLOCATION" => Ok(Self::FailedResourcesAllocation),
				"OUT_OF_CREDIT" => Ok(Self::OutOfCredit),
				"PDU_SESSION_STATUS" => Ok(Self::PduSessionStatus),
				"PLMN_CHG" => Ok(Self::PlmnChg),
				"QOS_MONITORING" => Ok(Self::QosMonitoring),
				"QOS_NOTIF" => Ok(Self::QosNotif),
				"RAN_NAS_CAUSE" => Ok(Self::RanNasCause),
				"REALLOCATION_OF_CREDIT" => Ok(Self::ReallocationOfCredit),
				"SAT_CATEGORY_CHG" => Ok(Self::SatCategoryChg),
				"SUCCESSFUL_QOS_UPDATE" => Ok(Self::SuccessfulQosUpdate),
				"SUCCESSFUL_RESOURCES_ALLOCATION" => Ok(Self::SuccessfulResourcesAllocation),
				"TSN_BRIDGE_INFO" => Ok(Self::TsnBridgeInfo),
				"UP_PATH_CHG_FAILURE" => Ok(Self::UpPathChgFailure),
				"USAGE_REPORT" => Ok(Self::UsageReport),
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

	/// Describes the event information delivered in the notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the event information delivered in the
	/// notification.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "event": {
	///      "$ref": "#/components/schemas/AfEvent"
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
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
	pub struct AfEventNotification {
		pub event: AfEvent,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub flows: Vec<Flows>,
	}

	impl From<&AfEventNotification> for AfEventNotification {
		fn from(value: &AfEventNotification) -> Self {
			value.clone()
		}
	}

	/// Describes the event information delivered in the subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the event information delivered in the
	/// subscription.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "event": {
	///      "$ref": "#/components/schemas/AfEvent"
	///    },
	///    "notifMethod": {
	///      "$ref": "#/components/schemas/AfNotifMethod"
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "waitTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfEventSubscription {
		pub event: AfEvent,
		#[serde(
			rename = "notifMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_method: Option<AfNotifMethod>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
		#[serde(rename = "waitTime", default, skip_serializing_if = "Option::is_none")]
		pub wait_time: Option<DurationSec>,
	}

	impl From<&AfEventSubscription> for AfEventSubscription {
		fn from(value: &AfEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Represents the notification methods that can be subscribed for an event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the notification methods that can be
	/// subscribed for an event.",
	///  "type": "string",
	///  "enum": [
	///    "EVENT_DETECTION",
	///    "ONE_TIME",
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
	pub enum AfNotifMethod {
		#[default]
		#[serde(rename = "EVENT_DETECTION")]
		EventDetection,
		#[serde(rename = "ONE_TIME")]
		OneTime,
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AfNotifMethod> for AfNotifMethod {
		fn from(value: &AfNotifMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for AfNotifMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::EventDetection => "EVENT_DETECTION".to_string(),
				Self::OneTime => "ONE_TIME".to_string(),
				Self::Periodic => "PERIODIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AfNotifMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EVENT_DETECTION" => Ok(Self::EventDetection),
				"ONE_TIME" => Ok(Self::OneTime),
				"PERIODIC" => Ok(Self::Periodic),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AfNotifMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AfNotifMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AfNotifMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the information that the AF requested to be exposed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the information that the AF requested to be
	/// exposed.",
	///  "type": "string",
	///  "enum": [
	///    "UE_IDENTITY"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AfRequestedData {
		#[default]
		#[serde(rename = "UE_IDENTITY")]
		UeIdentity,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AfRequestedData> for AfRequestedData {
		fn from(value: &AfRequestedData) -> Self {
			value.clone()
		}
	}

	impl ToString for AfRequestedData {
		fn to_string(&self) -> String {
			match *self {
				Self::UeIdentity => "UE_IDENTITY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AfRequestedData {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_IDENTITY" => Ok(Self::UeIdentity),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AfRequestedData {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AfRequestedData {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AfRequestedData {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the event information delivered in the subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the event information delivered in the
	/// subscription.",
	///  "type": "object",
	///  "properties": {
	///    "addrPreserInd": {
	///      "type": "boolean"
	///    },
	///    "appReloc": {
	///      "type": "boolean"
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "easRedisInd": {
	///      "description": "Indicates the EAS rediscovery is required.",
	///      "type": "boolean"
	///    },
	///    "maxAllowedUpLat": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "routeToLocs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RouteToLocation"
	///      },
	///      "minItems": 1
	///    },
	///    "simConnInd": {
	///      "description": "Indicates whether simultaneous connectivity should
	/// be temporarily maintained for the source and target PSA.",
	///      "type": "boolean"
	///    },
	///    "simConnTerm": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "spVal": {
	///      "$ref": "#/components/schemas/SpatialValidity"
	///    },
	///    "tempVals": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TemporalValidity"
	///      },
	///      "minItems": 1
	///    },
	///    "upPathChgSub": {
	///      "$ref": "#/components/schemas/UpPathChgEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfRoutingRequirement {
		#[serde(
			rename = "addrPreserInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub addr_preser_ind: Option<bool>,
		#[serde(rename = "appReloc", default, skip_serializing_if = "Option::is_none")]
		pub app_reloc: Option<bool>,
		/// Contains EAS IP replacement information.
		#[serde(
			rename = "easIpReplaceInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eas_ip_replace_infos: Vec<EasIpReplacementInfo>,
		/// Indicates the EAS rediscovery is required.
		#[serde(
			rename = "easRedisInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eas_redis_ind: Option<bool>,
		#[serde(
			rename = "maxAllowedUpLat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_allowed_up_lat: Option<Uinteger>,
		#[serde(rename = "routeToLocs", default, skip_serializing_if = "Vec::is_empty")]
		pub route_to_locs: Vec<RouteToLocation>,
		/// Indicates whether simultaneous connectivity should be temporarily
		/// maintained for the source and target PSA.
		#[serde(
			rename = "simConnInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_ind: Option<bool>,
		#[serde(
			rename = "simConnTerm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_term: Option<DurationSec>,
		#[serde(rename = "spVal", default, skip_serializing_if = "Option::is_none")]
		pub sp_val: Option<SpatialValidity>,
		#[serde(rename = "tempVals", default, skip_serializing_if = "Vec::is_empty")]
		pub temp_vals: Vec<TemporalValidity>,
		#[serde(
			rename = "upPathChgSub",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_path_chg_sub: Option<UpPathChgEvent>,
	}

	impl From<&AfRoutingRequirement> for AfRoutingRequirement {
		fn from(value: &AfRoutingRequirement) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the AfRoutingRequirement
	/// data type, but with the OpenAPI nullable property set to true and the
	/// spVal and tempVals attributes defined as removable.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// AfRoutingRequirement data type, but with the OpenAPI nullable property
	/// set to true and the spVal and tempVals attributes defined as
	/// removable.\n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "addrPreserInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "appReloc": {
	///      "type": "boolean"
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "easRedisInd": {
	///      "description": "Indicates the EAS rediscovery is required.",
	///      "type": "boolean"
	///    },
	///    "maxAllowedUpLat": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    },
	///    "routeToLocs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RouteToLocation"
	///      },
	///      "minItems": 1
	///    },
	///    "simConnInd": {
	///      "description": "Indicates whether simultaneous connectivity should
	/// be temporarily maintained for the source and target PSA.",
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "simConnTerm": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "spVal": {
	///      "$ref": "#/components/schemas/SpatialValidityRm"
	///    },
	///    "tempVals": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/TemporalValidity"
	///      },
	///      "minItems": 1
	///    },
	///    "upPathChgSub": {
	///      "$ref": "#/components/schemas/UpPathChgEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfRoutingRequirementRm(pub Option<AfRoutingRequirementRmInner>);

	impl ::std::ops::Deref for AfRoutingRequirementRm {
		type Target = Option<AfRoutingRequirementRmInner>;
		fn deref(&self) -> &Option<AfRoutingRequirementRmInner> {
			&self.0
		}
	}

	impl From<AfRoutingRequirementRm> for Option<AfRoutingRequirementRmInner> {
		fn from(value: AfRoutingRequirementRm) -> Self {
			value.0
		}
	}

	impl From<&AfRoutingRequirementRm> for AfRoutingRequirementRm {
		fn from(value: &AfRoutingRequirementRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<AfRoutingRequirementRmInner>> for AfRoutingRequirementRm {
		fn from(value: Option<AfRoutingRequirementRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the AfRoutingRequirement
	/// data type, but with the OpenAPI nullable property set to true and the
	/// spVal and tempVals attributes defined as removable.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// AfRoutingRequirement data type, but with the OpenAPI nullable property
	/// set to true and the spVal and tempVals attributes defined as
	/// removable.\n",
	///  "type": "object",
	///  "properties": {
	///    "addrPreserInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "appReloc": {
	///      "type": "boolean"
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "easRedisInd": {
	///      "description": "Indicates the EAS rediscovery is required.",
	///      "type": "boolean"
	///    },
	///    "maxAllowedUpLat": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    },
	///    "routeToLocs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RouteToLocation"
	///      },
	///      "minItems": 1
	///    },
	///    "simConnInd": {
	///      "description": "Indicates whether simultaneous connectivity should
	/// be temporarily maintained for the source and target PSA.",
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "simConnTerm": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "spVal": {
	///      "$ref": "#/components/schemas/SpatialValidityRm"
	///    },
	///    "tempVals": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/TemporalValidity"
	///      },
	///      "minItems": 1
	///    },
	///    "upPathChgSub": {
	///      "$ref": "#/components/schemas/UpPathChgEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfRoutingRequirementRmInner {
		#[serde(
			rename = "addrPreserInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub addr_preser_ind: Option<bool>,
		#[serde(rename = "appReloc", default, skip_serializing_if = "Option::is_none")]
		pub app_reloc: Option<bool>,
		/// Contains EAS IP replacement information.
		#[serde(
			rename = "easIpReplaceInfos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eas_ip_replace_infos: Option<Vec<EasIpReplacementInfo>>,
		/// Indicates the EAS rediscovery is required.
		#[serde(
			rename = "easRedisInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eas_redis_ind: Option<bool>,
		#[serde(
			rename = "maxAllowedUpLat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_allowed_up_lat: Option<UintegerRm>,
		#[serde(
			rename = "routeToLocs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub route_to_locs: Option<Vec<RouteToLocation>>,
		/// Indicates whether simultaneous connectivity should be temporarily
		/// maintained for the source and target PSA.
		#[serde(
			rename = "simConnInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_ind: Option<bool>,
		#[serde(
			rename = "simConnTerm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_term: Option<DurationSecRm>,
		#[serde(rename = "spVal", default, skip_serializing_if = "Option::is_none")]
		pub sp_val: Option<SpatialValidityRm>,
		#[serde(rename = "tempVals", default, skip_serializing_if = "Option::is_none")]
		pub temp_vals: Option<Vec<TemporalValidity>>,
		#[serde(
			rename = "upPathChgSub",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_path_chg_sub: Option<UpPathChgEvent>,
	}

	impl From<&AfRoutingRequirementRmInner> for AfRoutingRequirementRmInner {
		fn from(value: &AfRoutingRequirementRmInner) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - NO_INFORMATION: Indicate that no information about the AF signalling
	///   protocol is being provided.
	/// - SIP: Indicate that the signalling protocol is Session Initiation
	///   Protocol.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- NO_INFORMATION: Indicate that no
	/// information about the AF signalling protocol is being provided. \n- SIP:
	/// Indicate that the signalling protocol is Session Initiation
	/// Protocol.\n",
	///  "anyOf": [
	///    {
	///      "type": "string",
	///      "enum": [
	///        "NO_INFORMATION",
	///        "SIP"
	///      ]
	///    },
	///    {
	///      "$ref": "#/components/schemas/NullValue"
	///    },
	///    {
	///      "description": "This string provides forward-compatibility with
	/// future extensions to the enumeration but is not used to encode content
	/// defined in the present version of this API.\n",
	///      "type": "string"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfSigProtocol {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AfSigProtocolSubtype0>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<NullValue>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_2: Option<String>,
	}

	impl From<&AfSigProtocol> for AfSigProtocol {
		fn from(value: &AfSigProtocol) -> Self {
			value.clone()
		}
	}

	/// AfSigProtocolSubtype0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NO_INFORMATION",
	///    "SIP"
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
	pub enum AfSigProtocolSubtype0 {
		#[default]
		#[serde(rename = "NO_INFORMATION")]
		NoInformation,
		#[serde(rename = "SIP")]
		Sip,
	}

	impl From<&AfSigProtocolSubtype0> for AfSigProtocolSubtype0 {
		fn from(value: &AfSigProtocolSubtype0) -> Self {
			value.clone()
		}
	}

	impl ToString for AfSigProtocolSubtype0 {
		fn to_string(&self) -> String {
			match *self {
				Self::NoInformation => "NO_INFORMATION".to_string(),
				Self::Sip => "SIP".to_string(),
			}
		}
	}

	impl std::str::FromStr for AfSigProtocolSubtype0 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NO_INFORMATION" => Ok(Self::NoInformation),
				"SIP" => Ok(Self::Sip),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AfSigProtocolSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AfSigProtocolSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AfSigProtocolSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains an alternative QoS related parameter set.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an alternative QoS related parameter set.",
	///  "type": "object",
	///  "required": [
	///    "altQosParamSetRef"
	///  ],
	///  "properties": {
	///    "altQosParamSetRef": {
	///      "description": "Reference to this alternative QoS related parameter
	/// set.",
	///      "type": "string"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "gbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "pdb": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AlternativeServiceRequirementsData {
		/// Reference to this alternative QoS related parameter set.
		#[serde(rename = "altQosParamSetRef")]
		pub alt_qos_param_set_ref: String,
		#[serde(rename = "gbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_dl: Option<BitRate>,
		#[serde(rename = "gbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_ul: Option<BitRate>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pdb: Option<PacketDelBudget>,
	}

	impl From<&AlternativeServiceRequirementsData> for AlternativeServiceRequirementsData {
		fn from(value: &AlternativeServiceRequirementsData) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - SAC_CH: Service Area Coverage Change
	/// - PDUID_CH: The PDUID assigned to a UE for the UE ProSe Policies changed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- SAC_CH: Service Area Coverage
	/// Change\n- PDUID_CH: The PDUID assigned to a UE for the UE ProSe Policies
	/// changed\n",
	///  "type": "string",
	///  "enum": [
	///    "SAC_CH",
	///    "PDUID_CH"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AmEvent {
		#[default]
		#[serde(rename = "SAC_CH")]
		SacCh,
		#[serde(rename = "PDUID_CH")]
		PduidCh,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AmEvent> for AmEvent {
		fn from(value: &AmEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for AmEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::SacCh => "SAC_CH".to_string(),
				Self::PduidCh => "PDUID_CH".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AmEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SAC_CH" => Ok(Self::SacCh),
				"PDUID_CH" => Ok(Self::PduidCh),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// This data type contains the event identifier and the related event
	/// reporting information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type contains the event identifier and the
	/// related event reporting information.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "event": {
	///      "$ref": "#/components/schemas/AmEvent"
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
	///    "notifMethod": {
	///      "$ref": "#/components/schemas/NotificationMethod"
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmEventData {
		pub event: AmEvent,
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
		#[serde(
			rename = "notifMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_method: Option<NotificationMethod>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
	}

	impl From<&AmEventData> for AmEventData {
		fn from(value: &AmEventData) -> Self {
			value.clone()
		}
	}

	/// Describes the notification of a subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the notification of a subscription.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "appliedCov": {
	///      "$ref": "#/components/schemas/ServiceAreaCoverageInfo"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/AmEvent"
	///    },
	///    "pduidInfo": {
	///      "$ref": "#/components/schemas/PduidInformation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmEventNotification {
		#[serde(
			rename = "appliedCov",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub applied_cov: Option<ServiceAreaCoverageInfo>,
		pub event: AmEvent,
		#[serde(rename = "pduidInfo", default, skip_serializing_if = "Option::is_none")]
		pub pduid_info: Option<PduidInformation>,
	}

	impl From<&AmEventNotification> for AmEventNotification {
		fn from(value: &AmEventNotification) -> Self {
			value.clone()
		}
	}

	/// Describes the notification about the events occurred within an
	/// Individual Application AM Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the notification about the events occurred
	/// within an Individual Application AM Context resource.",
	///  "type": "object",
	///  "required": [
	///    "repEvents"
	///  ],
	///  "properties": {
	///    "appAmContextId": {
	///      "description": "Contains the AM Policy Events Subscription resource
	/// identifier related to the event notification.",
	///      "type": "string"
	///    },
	///    "repEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmEventNotification"
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
	pub struct AmEventsNotification {
		/// Contains the AM Policy Events Subscription resource identifier
		/// related to the event notification.
		#[serde(
			rename = "appAmContextId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub app_am_context_id: Option<String>,
		#[serde(rename = "repEvents")]
		pub rep_events: Vec<AmEventNotification>,
	}

	impl From<&AmEventsNotification> for AmEventsNotification {
		fn from(value: &AmEventsNotification) -> Self {
			value.clone()
		}
	}

	/// It represents the AM Policy Events Subscription subresource and
	/// identifies the events the application subscribes to.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It represents the AM Policy Events Subscription subresource and identifies the events the application subscribes to.",
	///  "type": "object",
	///  "required": [
	///    "eventNotifUri"
	///  ],
	///  "properties": {
	///    "eventNotifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmEventData"
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
	pub struct AmEventsSubscData {
		#[serde(rename = "eventNotifUri")]
		pub event_notif_uri: Uri,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub events: Vec<AmEventData>,
	}

	impl From<&AmEventsSubscData> for AmEventsSubscData {
		fn from(value: &AmEventsSubscData) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the AmEventsSubscData but
	/// with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the AmEventsSubscData but with the OpenAPI nullable property set to true.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "eventNotifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmEventData"
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
	pub struct AmEventsSubscDataRm(pub Option<AmEventsSubscDataRmInner>);

	impl ::std::ops::Deref for AmEventsSubscDataRm {
		type Target = Option<AmEventsSubscDataRmInner>;
		fn deref(&self) -> &Option<AmEventsSubscDataRmInner> {
			&self.0
		}
	}

	impl From<AmEventsSubscDataRm> for Option<AmEventsSubscDataRmInner> {
		fn from(value: AmEventsSubscDataRm) -> Self {
			value.0
		}
	}

	impl From<&AmEventsSubscDataRm> for AmEventsSubscDataRm {
		fn from(value: &AmEventsSubscDataRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<AmEventsSubscDataRmInner>> for AmEventsSubscDataRm {
		fn from(value: Option<AmEventsSubscDataRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the AmEventsSubscData but
	/// with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the AmEventsSubscData but with the OpenAPI nullable property set to true.",
	///  "type": "object",
	///  "properties": {
	///    "eventNotifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmEventData"
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
	pub struct AmEventsSubscDataRmInner {
		#[serde(
			rename = "eventNotifUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_notif_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub events: Vec<AmEventData>,
	}

	impl From<&AmEventsSubscDataRmInner> for AmEventsSubscDataRmInner {
		fn from(value: &AmEventsSubscDataRmInner) -> Self {
			value.clone()
		}
	}

	/// Identifies the events the application subscribes to within an AM Policy
	/// Events Subscription subresource data. It may contain the notification of
	/// the already met events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the events the application subscribes to
	/// within an AM Policy Events Subscription subresource data. It may contain
	/// the notification of the already met events.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AmEventsSubscData"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AmEventsNotification"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AmEventsSubscRespData {
		#[default]
		SubscData(AmEventsSubscData),
		Notification(AmEventsNotification),
	}

	impl From<&AmEventsSubscRespData> for AmEventsSubscRespData {
		fn from(value: &AmEventsSubscRespData) -> Self {
			value.clone()
		}
	}

	impl From<AmEventsSubscData> for AmEventsSubscRespData {
		fn from(value: AmEventsSubscData) -> Self {
			Self::SubscData(value)
		}
	}

	impl From<AmEventsNotification> for AmEventsSubscRespData {
		fn from(value: AmEventsNotification) -> Self {
			Self::Notification(value)
		}
	}

	/// Represents the current applicable values corresponding to the policy
	/// control request triggers.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the current applicable values corresponding
	/// to the policy control request triggers.\n",
	///  "type": "object",
	///  "properties": {
	///    "accessTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the 3GPP access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      }
	///    },
	///    "n3gAllowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the Non-3GPP
	/// access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      }
	///    },
	///    "praStatuses": {
	///      "description": "Contains the UE presence statuses for tracking
	/// areas. The praId attribute within the PresenceInfo data type is the key
	/// of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "ratTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      }
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmRequestedValueRep {
		#[serde(rename = "accessTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub access_types: Vec<AccessType>,
		/// array of allowed S-NSSAIs for the 3GPP access.
		#[serde(
			rename = "allowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_snssais: Vec<Snssai>,
		/// array of allowed S-NSSAIs for the Non-3GPP access.
		#[serde(
			rename = "n3gAllowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n3g_allowed_snssais: Vec<Snssai>,
		/// Contains the UE presence statuses for tracking areas. The praId
		/// attribute within the PresenceInfo data type is the key of the map.
		#[serde(
			rename = "praStatuses",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_statuses: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "ratTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub rat_types: Vec<RatType>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
	}

	impl From<&AmRequestedValueRep> for AmRequestedValueRep {
		fn from(value: &AmRequestedValueRep) -> Self {
			value.clone()
		}
	}

	/// It represents the cause values that the PCF should report when
	/// requesting to an NF service consumer the deletion of an "AF application
	/// AM context" resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It represents the cause values that the PCF should
	/// report when requesting to an NF service consumer the deletion of an \"AF
	/// application AM context\" resource.",
	///  "type": "string",
	///  "enum": [
	///    "UE_DEREGISTERED",
	///    "UNSPECIFIED",
	///    "INSUFFICIENT_RESOURCES"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AmTerminationCause {
		#[default]
		#[serde(rename = "UE_DEREGISTERED")]
		UeDeregistered,
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(rename = "INSUFFICIENT_RESOURCES")]
		InsufficientResources,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AmTerminationCause> for AmTerminationCause {
		fn from(value: &AmTerminationCause) -> Self {
			value.clone()
		}
	}

	impl ToString for AmTerminationCause {
		fn to_string(&self) -> String {
			match *self {
				Self::UeDeregistered => "UE_DEREGISTERED".to_string(),
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::InsufficientResources => "INSUFFICIENT_RESOURCES".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AmTerminationCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_DEREGISTERED" => Ok(Self::UeDeregistered),
				"UNSPECIFIED" => Ok(Self::Unspecified),
				"INSUFFICIENT_RESOURCES" => Ok(Self::InsufficientResources),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmTerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmTerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmTerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Includes information related to the termination of the Individual
	/// Application AM Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Includes information related to the termination of the
	/// Individual Application AM Context resource.",
	///  "type": "object",
	///  "required": [
	///    "appAmContextId",
	///    "termCause"
	///  ],
	///  "properties": {
	///    "appAmContextId": {
	///      "description": "Contains the Individual application AM context
	/// resource identifier related to the termination notification.",
	///      "type": "string"
	///    },
	///    "termCause": {
	///      "$ref": "#/components/schemas/AmTerminationCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmTerminationInfo {
		/// Contains the Individual application AM context resource identifier
		/// related to the termination notification.
		#[serde(rename = "appAmContextId")]
		pub app_am_context_id: String,
		#[serde(rename = "termCause")]
		pub term_cause: AmTerminationCause,
	}

	impl From<&AmTerminationInfo> for AmTerminationInfo {
		fn from(value: &AmTerminationInfo) -> Self {
			value.clone()
		}
	}

	/// Describes the address of the access network gateway control node.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the address of the access network gateway
	/// control node.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "anGwIpv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "anGwIpv6Addr"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "anGwIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "anGwIpv6Addr": {
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
	pub enum AnGwAddress {
		#[default]
		Variant0 {
			#[serde(rename = "anGwIpv4Addr")]
			an_gw_ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "anGwIpv6Addr")]
			an_gw_ipv6_addr: Ipv6Addr,
		},
	}

	impl From<&AnGwAddress> for AnGwAddress {
		fn from(value: &AnGwAddress) -> Self {
			value.clone()
		}
	}

	/// Represents an Individual Application AM Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual Application AM Context
	/// resource.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "highThruInd"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "covReq"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "required": [
	///        "asTimeDisParam"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "evSubsc"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "supi",
	///    "termNotifUri"
	///  ],
	///  "properties": {
	///    "asTimeDisParam": {
	///      "$ref": "#/components/schemas/AsTimeDistributionParam"
	///    },
	///    "covReq": {
	///      "description": "Identifies a list of Tracking Areas per serving
	/// network where service is allowed.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceAreaCoverageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "evSubsc": {
	///      "$ref": "#/components/schemas/AmEventsSubscData"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "highThruInd": {
	///      "description": "Indicates whether high throughput is desired for
	/// the indicated UE traffic.",
	///      "type": "boolean"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "termNotifUri": {
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
	pub enum AppAmContextData {
		#[default]
		Variant0(AppAmContextDataVariant0),
		Variant1 {
			#[serde(rename = "asTimeDisParam")]
			as_time_dis_param: AsTimeDistributionParam,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			expiry: Option<DurationSec>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			supi: Supi,
			#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
			supp_feat: Option<SupportedFeatures>,
			#[serde(rename = "termNotifUri")]
			term_notif_uri: Uri,
		},
		Variant2 {
			#[serde(rename = "evSubsc")]
			ev_subsc: AmEventsSubscData,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			expiry: Option<DurationSec>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			supi: Supi,
			#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
			supp_feat: Option<SupportedFeatures>,
			#[serde(rename = "termNotifUri")]
			term_notif_uri: Uri,
		},
	}

	impl From<&AppAmContextData> for AppAmContextData {
		fn from(value: &AppAmContextData) -> Self {
			value.clone()
		}
	}

	impl From<AppAmContextDataVariant0> for AppAmContextData {
		fn from(value: AppAmContextDataVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	/// AppAmContextDataVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "type": "object",
	///      "required": [
	///        "supi",
	///        "termNotifUri"
	///      ],
	///      "properties": {
	///        "asTimeDisParam": {
	///          "$ref": "#/components/schemas/AsTimeDistributionParam"
	///        },
	///        "covReq": {
	///          "description": "Identifies a list of Tracking Areas per serving
	/// network where service is allowed.",
	///          "type": "array",
	///          "items": {
	///            "$ref": "#/components/schemas/ServiceAreaCoverageInfo"
	///          },
	///          "minItems": 1
	///        },
	///        "evSubsc": {
	///          "$ref": "#/components/schemas/AmEventsSubscData"
	///        },
	///        "expiry": {
	///          "$ref": "#/components/schemas/DurationSec"
	///        },
	///        "gpsi": {
	///          "$ref": "#/components/schemas/Gpsi"
	///        },
	///        "highThruInd": {
	///          "description": "Indicates whether high throughput is desired
	/// for the indicated UE traffic.",
	///          "type": "boolean"
	///        },
	///        "supi": {
	///          "$ref": "#/components/schemas/Supi"
	///        },
	///        "suppFeat": {
	///          "$ref": "#/components/schemas/SupportedFeatures"
	///        },
	///        "termNotifUri": {
	///          "$ref": "#/components/schemas/Uri"
	///        }
	///      }
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "highThruInd"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "covReq"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "asTimeDisParam"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "evSubsc"
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
	pub enum AppAmContextDataVariant0 {
		#[default]
		None,
	}

	impl From<&AppAmContextDataVariant0> for AppAmContextDataVariant0 {
		fn from(value: &AppAmContextDataVariant0) -> Self {
			value.clone()
		}
	}

	/// It represents a response to a modification or creation request of an
	/// Individual Application AM resource. It may contain the notification of
	/// the already met events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It represents a response to a modification or creation
	/// request of an Individual Application AM resource. It may contain the
	/// notification of the already met events.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AppAmContextData"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AmEventsNotification"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppAmContextRespData {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AppAmContextData>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<AmEventsNotification>,
	}

	impl From<&AppAmContextRespData> for AppAmContextRespData {
		fn from(value: &AppAmContextRespData) -> Self {
			value.clone()
		}
	}

	/// Describes the modifications to an Individual Application AM resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the modifications to an Individual
	/// Application AM resource.",
	///  "type": "object",
	///  "properties": {
	///    "asTimeDisParam": {
	///      "$ref": "#/components/schemas/AsTimeDistributionParam"
	///    },
	///    "covReq": {
	///      "description": "Identifies a list of Tracking Areas per serving
	/// network where service is allowed.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceAreaCoverageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "evSubsc": {
	///      "$ref": "#/components/schemas/AmEventsSubscDataRm"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "highThruInd": {
	///      "description": "Indicates whether high throughput is desired for
	/// the indicated UE traffic.",
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "termNotifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppAmContextUpdateData {
		#[serde(
			rename = "asTimeDisParam",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub as_time_dis_param: Option<AsTimeDistributionParam>,
		/// Identifies a list of Tracking Areas per serving network where
		/// service is allowed.
		#[serde(rename = "covReq", default, skip_serializing_if = "Option::is_none")]
		pub cov_req: Option<Vec<ServiceAreaCoverageInfo>>,
		#[serde(rename = "evSubsc", default, skip_serializing_if = "Option::is_none")]
		pub ev_subsc: Option<AmEventsSubscDataRm>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DurationSecRm>,
		/// Indicates whether high throughput is desired for the indicated UE
		/// traffic.
		#[serde(
			rename = "highThruInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub high_thru_ind: Option<bool>,
		#[serde(
			rename = "termNotifUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub term_notif_uri: Option<Uri>,
	}

	impl From<&AppAmContextUpdateData> for AppAmContextUpdateData {
		fn from(value: &AppAmContextUpdateData) -> Self {
			value.clone()
		}
	}

	/// Contains the detected application's traffic information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the detected application's traffic
	/// information.",
	///  "type": "object",
	///  "required": [
	///    "appId"
	///  ],
	///  "properties": {
	///    "appId": {
	///      "description": "A reference to the application detection filter
	/// configured at the UPF",
	///      "type": "string"
	///    },
	///    "instanceId": {
	///      "description": "Identifier sent by the SMF in order to allow
	/// correlation of application Start and Stop events to the specific service
	/// data flow description, if service data flow descriptions are
	/// deducible.\n",
	///      "type": "string"
	///    },
	///    "sdfDescriptions": {
	///      "description": "Contains the detected service data flow
	/// descriptions if they are deducible.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowInformation"
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
	pub struct AppDetectionInfo {
		/// A reference to the application detection filter configured at the
		/// UPF
		#[serde(rename = "appId")]
		pub app_id: String,
		/// Identifier sent by the SMF in order to allow correlation of
		/// application Start and Stop events to the specific service data flow
		/// description, if service data flow descriptions are deducible.
		#[serde(
			rename = "instanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub instance_id: Option<String>,
		/// Contains the detected service data flow descriptions if they are
		/// deducible.
		#[serde(
			rename = "sdfDescriptions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sdf_descriptions: Vec<FlowInformation>,
	}

	impl From<&AppDetectionInfo> for AppDetectionInfo {
		fn from(value: &AppDetectionInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the notification type for Application Detection Control.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the notification type for Application
	/// Detection Control.",
	///  "type": "string",
	///  "enum": [
	///    "APP_START",
	///    "APP_STOP"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AppDetectionNotifType {
		#[default]
		#[serde(rename = "APP_START")]
		AppStart,
		#[serde(rename = "APP_STOP")]
		AppStop,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AppDetectionNotifType> for AppDetectionNotifType {
		fn from(value: &AppDetectionNotifType) -> Self {
			value.clone()
		}
	}

	impl ToString for AppDetectionNotifType {
		fn to_string(&self) -> String {
			match *self {
				Self::AppStart => "APP_START".to_string(),
				Self::AppStop => "APP_STOP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AppDetectionNotifType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"APP_START" => Ok(Self::AppStart),
				"APP_STOP" => Ok(Self::AppStop),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AppDetectionNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AppDetectionNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AppDetectionNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the start or stop of the detected application traffic and the
	/// application identifier of the detected application traffic.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the start or stop of the detected application
	/// traffic and the application identifier of the detected application
	/// traffic.",
	///  "type": "object",
	///  "required": [
	///    "adNotifType",
	///    "afAppId"
	///  ],
	///  "properties": {
	///    "adNotifType": {
	///      "$ref": "#/components/schemas/AppDetectionNotifType"
	///    },
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppDetectionReport {
		#[serde(rename = "adNotifType")]
		pub ad_notif_type: AppDetectionNotifType,
		#[serde(rename = "afAppId")]
		pub af_app_id: AfAppId,
	}

	impl From<&AppDetectionReport> for AppDetectionReport {
		fn from(value: &AppDetectionReport) -> Self {
			value.clone()
		}
	}

	/// Represents an Individual Application Session Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual Application Session Context
	/// resource.",
	///  "type": "object",
	///  "properties": {
	///    "ascReqData": {
	///      "$ref": "#/components/schemas/AppSessionContextReqData"
	///    },
	///    "ascRespData": {
	///      "$ref": "#/components/schemas/AppSessionContextRespData"
	///    },
	///    "evsNotif": {
	///      "$ref": "#/components/schemas/EventsNotification"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppSessionContext {
		#[serde(
			rename = "ascReqData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub asc_req_data: Option<AppSessionContextReqData>,
		#[serde(
			rename = "ascRespData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub asc_resp_data: Option<AppSessionContextRespData>,
		#[serde(rename = "evsNotif", default, skip_serializing_if = "Option::is_none")]
		pub evs_notif: Option<EventsNotification>,
	}

	impl From<&AppSessionContext> for AppSessionContext {
		fn from(value: &AppSessionContext) -> Self {
			value.clone()
		}
	}

	/// Identifies the service requirements of an Individual Application Session
	/// Context.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the service requirements of an Individual
	/// Application Session Context.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ueIpv4"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ueIpv6"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ueMac"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "notifUri",
	///    "suppFeat"
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "afChargId": {
	///      "$ref": "#/components/schemas/ApplicationChargingId"
	///    },
	///    "afReqData": {
	///      "$ref": "#/components/schemas/AfRequestedData"
	///    },
	///    "afRoutReq": {
	///      "$ref": "#/components/schemas/AfRoutingRequirement"
	///    },
	///    "aspId": {
	///      "$ref": "#/components/schemas/AspId"
	///    },
	///    "bdtRefId": {
	///      "$ref": "#/components/schemas/BdtReferenceId"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "evSubsc": {
	///      "$ref": "#/components/schemas/EventsSubscReqData"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "ipDomain": {
	///      "type": "string"
	///    },
	///    "mcVideoId": {
	///      "description": "Indication of MCVideo service request.",
	///      "type": "string"
	///    },
	///    "mcpttId": {
	///      "description": "Indication of MCPTT service request.",
	///      "type": "string"
	///    },
	///    "mcsId": {
	///      "description": "Indication of MCS service request.",
	///      "type": "string"
	///    },
	///    "medComponents": {
	///      "description": "Contains media component information. The key of
	/// the map is the medCompN attribute.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaComponent"
	///      }
	///    },
	///    "mpsAction": {
	///      "$ref": "#/components/schemas/MpsAction"
	///    },
	///    "mpsId": {
	///      "description": "Indication of MPS service request.",
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "preemptControlInfo": {
	///      "$ref": "#/components/schemas/PreemptionControlInformation"
	///    },
	///    "resPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "servInfStatus": {
	///      "$ref": "#/components/schemas/ServiceInfoStatus"
	///    },
	///    "servUrn": {
	///      "$ref": "#/components/schemas/ServiceUrn"
	///    },
	///    "sliceInfo": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "sponId": {
	///      "$ref": "#/components/schemas/SponId"
	///    },
	///    "sponStatus": {
	///      "$ref": "#/components/schemas/SponsoringStatus"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "ueIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "ueMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AppSessionContextReqData {
		#[default]
		Variant0 {
			#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
			af_app_id: Option<AfAppId>,
			#[serde(rename = "afChargId", default, skip_serializing_if = "Option::is_none")]
			af_charg_id: Option<ApplicationChargingId>,
			#[serde(rename = "afReqData", default, skip_serializing_if = "Option::is_none")]
			af_req_data: Option<AfRequestedData>,
			#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
			af_rout_req: Option<AfRoutingRequirement>,
			#[serde(rename = "aspId", default, skip_serializing_if = "Option::is_none")]
			asp_id: Option<AspId>,
			#[serde(rename = "bdtRefId", default, skip_serializing_if = "Option::is_none")]
			bdt_ref_id: Option<BdtReferenceId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "evSubsc", default, skip_serializing_if = "Option::is_none")]
			ev_subsc: Option<EventsSubscReqData>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			/// Indication of MCVideo service request.
			#[serde(rename = "mcVideoId", default, skip_serializing_if = "Option::is_none")]
			mc_video_id: Option<String>,
			/// Indication of MCPTT service request.
			#[serde(rename = "mcpttId", default, skip_serializing_if = "Option::is_none")]
			mcptt_id: Option<String>,
			/// Indication of MCS service request.
			#[serde(rename = "mcsId", default, skip_serializing_if = "Option::is_none")]
			mcs_id: Option<String>,
			/// Contains media component information. The key of the map is the
			/// medCompN attribute.
			#[serde(
				rename = "medComponents",
				default,
				skip_serializing_if = "::std::collections::HashMap::is_empty"
			)]
			med_components: ::std::collections::HashMap<String, MediaComponent>,
			#[serde(rename = "mpsAction", default, skip_serializing_if = "Option::is_none")]
			mps_action: Option<MpsAction>,
			/// Indication of MPS service request.
			#[serde(rename = "mpsId", default, skip_serializing_if = "Option::is_none")]
			mps_id: Option<String>,
			#[serde(rename = "notifUri")]
			notif_uri: Uri,
			#[serde(
				rename = "preemptControlInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			preempt_control_info: Option<PreemptionControlInformation>,
			#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
			res_prio: Option<ReservPriority>,
			#[serde(
				rename = "servInfStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			serv_inf_status: Option<ServiceInfoStatus>,
			#[serde(rename = "servUrn", default, skip_serializing_if = "Option::is_none")]
			serv_urn: Option<ServiceUrn>,
			#[serde(rename = "sliceInfo", default, skip_serializing_if = "Option::is_none")]
			slice_info: Option<Snssai>,
			#[serde(rename = "sponId", default, skip_serializing_if = "Option::is_none")]
			spon_id: Option<SponId>,
			#[serde(
				rename = "sponStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			spon_status: Option<SponsoringStatus>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(rename = "suppFeat")]
			supp_feat: SupportedFeatures,
			#[serde(
				rename = "tsnBridgeManCont",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_bridge_man_cont: Option<BridgeManagementContainer>,
			#[serde(
				rename = "tsnPortManContDstt",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_port_man_cont_dstt: Option<PortManagementContainer>,
			#[serde(
				rename = "tsnPortManContNwtts",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
			#[serde(rename = "ueIpv4")]
			ue_ipv4: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
			af_app_id: Option<AfAppId>,
			#[serde(rename = "afChargId", default, skip_serializing_if = "Option::is_none")]
			af_charg_id: Option<ApplicationChargingId>,
			#[serde(rename = "afReqData", default, skip_serializing_if = "Option::is_none")]
			af_req_data: Option<AfRequestedData>,
			#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
			af_rout_req: Option<AfRoutingRequirement>,
			#[serde(rename = "aspId", default, skip_serializing_if = "Option::is_none")]
			asp_id: Option<AspId>,
			#[serde(rename = "bdtRefId", default, skip_serializing_if = "Option::is_none")]
			bdt_ref_id: Option<BdtReferenceId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "evSubsc", default, skip_serializing_if = "Option::is_none")]
			ev_subsc: Option<EventsSubscReqData>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			/// Indication of MCVideo service request.
			#[serde(rename = "mcVideoId", default, skip_serializing_if = "Option::is_none")]
			mc_video_id: Option<String>,
			/// Indication of MCPTT service request.
			#[serde(rename = "mcpttId", default, skip_serializing_if = "Option::is_none")]
			mcptt_id: Option<String>,
			/// Indication of MCS service request.
			#[serde(rename = "mcsId", default, skip_serializing_if = "Option::is_none")]
			mcs_id: Option<String>,
			/// Contains media component information. The key of the map is the
			/// medCompN attribute.
			#[serde(
				rename = "medComponents",
				default,
				skip_serializing_if = "::std::collections::HashMap::is_empty"
			)]
			med_components: ::std::collections::HashMap<String, MediaComponent>,
			#[serde(rename = "mpsAction", default, skip_serializing_if = "Option::is_none")]
			mps_action: Option<MpsAction>,
			/// Indication of MPS service request.
			#[serde(rename = "mpsId", default, skip_serializing_if = "Option::is_none")]
			mps_id: Option<String>,
			#[serde(rename = "notifUri")]
			notif_uri: Uri,
			#[serde(
				rename = "preemptControlInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			preempt_control_info: Option<PreemptionControlInformation>,
			#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
			res_prio: Option<ReservPriority>,
			#[serde(
				rename = "servInfStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			serv_inf_status: Option<ServiceInfoStatus>,
			#[serde(rename = "servUrn", default, skip_serializing_if = "Option::is_none")]
			serv_urn: Option<ServiceUrn>,
			#[serde(rename = "sliceInfo", default, skip_serializing_if = "Option::is_none")]
			slice_info: Option<Snssai>,
			#[serde(rename = "sponId", default, skip_serializing_if = "Option::is_none")]
			spon_id: Option<SponId>,
			#[serde(
				rename = "sponStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			spon_status: Option<SponsoringStatus>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(rename = "suppFeat")]
			supp_feat: SupportedFeatures,
			#[serde(
				rename = "tsnBridgeManCont",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_bridge_man_cont: Option<BridgeManagementContainer>,
			#[serde(
				rename = "tsnPortManContDstt",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_port_man_cont_dstt: Option<PortManagementContainer>,
			#[serde(
				rename = "tsnPortManContNwtts",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
			#[serde(rename = "ueIpv6")]
			ue_ipv6: Ipv6Addr,
		},
		Variant2 {
			#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
			af_app_id: Option<AfAppId>,
			#[serde(rename = "afChargId", default, skip_serializing_if = "Option::is_none")]
			af_charg_id: Option<ApplicationChargingId>,
			#[serde(rename = "afReqData", default, skip_serializing_if = "Option::is_none")]
			af_req_data: Option<AfRequestedData>,
			#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
			af_rout_req: Option<AfRoutingRequirement>,
			#[serde(rename = "aspId", default, skip_serializing_if = "Option::is_none")]
			asp_id: Option<AspId>,
			#[serde(rename = "bdtRefId", default, skip_serializing_if = "Option::is_none")]
			bdt_ref_id: Option<BdtReferenceId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "evSubsc", default, skip_serializing_if = "Option::is_none")]
			ev_subsc: Option<EventsSubscReqData>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			/// Indication of MCVideo service request.
			#[serde(rename = "mcVideoId", default, skip_serializing_if = "Option::is_none")]
			mc_video_id: Option<String>,
			/// Indication of MCPTT service request.
			#[serde(rename = "mcpttId", default, skip_serializing_if = "Option::is_none")]
			mcptt_id: Option<String>,
			/// Indication of MCS service request.
			#[serde(rename = "mcsId", default, skip_serializing_if = "Option::is_none")]
			mcs_id: Option<String>,
			/// Contains media component information. The key of the map is the
			/// medCompN attribute.
			#[serde(
				rename = "medComponents",
				default,
				skip_serializing_if = "::std::collections::HashMap::is_empty"
			)]
			med_components: ::std::collections::HashMap<String, MediaComponent>,
			#[serde(rename = "mpsAction", default, skip_serializing_if = "Option::is_none")]
			mps_action: Option<MpsAction>,
			/// Indication of MPS service request.
			#[serde(rename = "mpsId", default, skip_serializing_if = "Option::is_none")]
			mps_id: Option<String>,
			#[serde(rename = "notifUri")]
			notif_uri: Uri,
			#[serde(
				rename = "preemptControlInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			preempt_control_info: Option<PreemptionControlInformation>,
			#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
			res_prio: Option<ReservPriority>,
			#[serde(
				rename = "servInfStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			serv_inf_status: Option<ServiceInfoStatus>,
			#[serde(rename = "servUrn", default, skip_serializing_if = "Option::is_none")]
			serv_urn: Option<ServiceUrn>,
			#[serde(rename = "sliceInfo", default, skip_serializing_if = "Option::is_none")]
			slice_info: Option<Snssai>,
			#[serde(rename = "sponId", default, skip_serializing_if = "Option::is_none")]
			spon_id: Option<SponId>,
			#[serde(
				rename = "sponStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			spon_status: Option<SponsoringStatus>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(rename = "suppFeat")]
			supp_feat: SupportedFeatures,
			#[serde(
				rename = "tsnBridgeManCont",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_bridge_man_cont: Option<BridgeManagementContainer>,
			#[serde(
				rename = "tsnPortManContDstt",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			tsn_port_man_cont_dstt: Option<PortManagementContainer>,
			#[serde(
				rename = "tsnPortManContNwtts",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
			#[serde(rename = "ueMac")]
			ue_mac: MacAddr48,
		},
	}

	impl From<&AppSessionContextReqData> for AppSessionContextReqData {
		fn from(value: &AppSessionContextReqData) -> Self {
			value.clone()
		}
	}

	/// Describes the authorization data of an Individual Application Session
	/// Context created by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the authorization data of an Individual
	/// Application Session Context created by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "servAuthInfo": {
	///      "$ref": "#/components/schemas/ServAuthInfo"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "ueIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeIdentityInfo"
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
	pub struct AppSessionContextRespData {
		#[serde(
			rename = "servAuthInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_auth_info: Option<ServAuthInfo>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
		#[serde(rename = "ueIds", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_ids: Vec<UeIdentityInfo>,
	}

	impl From<&AppSessionContextRespData> for AppSessionContextRespData {
		fn from(value: &AppSessionContextRespData) -> Self {
			value.clone()
		}
	}

	/// Identifies the modifications to the "ascReqData" property of an
	/// Individual Application Session Context which may include the
	/// modifications to the sub-resource Events Subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the modifications to the \"ascReqData\"
	/// property of an Individual Application Session Context which may include
	/// the modifications to the sub-resource Events Subscription.\n",
	///  "type": "object",
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "afRoutReq": {
	///      "$ref": "#/components/schemas/AfRoutingRequirementRm"
	///    },
	///    "aspId": {
	///      "$ref": "#/components/schemas/AspId"
	///    },
	///    "bdtRefId": {
	///      "$ref": "#/components/schemas/BdtReferenceId"
	///    },
	///    "evSubsc": {
	///      "$ref": "#/components/schemas/EventsSubscReqDataRm"
	///    },
	///    "mcVideoId": {
	///      "description": "Indication of modification of MCVideo service.",
	///      "type": "string"
	///    },
	///    "mcpttId": {
	///      "description": "Indication of MCPTT service request.",
	///      "type": "string"
	///    },
	///    "mcsId": {
	///      "description": "Indication of MCS service request.",
	///      "type": "string"
	///    },
	///    "medComponents": {
	///      "description": "Contains media component information. The key of
	/// the map is the medCompN attribute.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaComponentRm"
	///      }
	///    },
	///    "mpsAction": {
	///      "$ref": "#/components/schemas/MpsAction"
	///    },
	///    "mpsId": {
	///      "description": "Indication of MPS service request.",
	///      "type": "string"
	///    },
	///    "preemptControlInfo": {
	///      "$ref": "#/components/schemas/PreemptionControlInformationRm"
	///    },
	///    "resPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "servInfStatus": {
	///      "$ref": "#/components/schemas/ServiceInfoStatus"
	///    },
	///    "sipForkInd": {
	///      "$ref": "#/components/schemas/SipForkingIndication"
	///    },
	///    "sponId": {
	///      "$ref": "#/components/schemas/SponId"
	///    },
	///    "sponStatus": {
	///      "$ref": "#/components/schemas/SponsoringStatus"
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
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
	pub struct AppSessionContextUpdateData {
		#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
		pub af_app_id: Option<AfAppId>,
		#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
		pub af_rout_req: Option<AfRoutingRequirementRm>,
		#[serde(rename = "aspId", default, skip_serializing_if = "Option::is_none")]
		pub asp_id: Option<AspId>,
		#[serde(rename = "bdtRefId", default, skip_serializing_if = "Option::is_none")]
		pub bdt_ref_id: Option<BdtReferenceId>,
		#[serde(rename = "evSubsc", default, skip_serializing_if = "Option::is_none")]
		pub ev_subsc: Option<EventsSubscReqDataRm>,
		/// Indication of modification of MCVideo service.
		#[serde(rename = "mcVideoId", default, skip_serializing_if = "Option::is_none")]
		pub mc_video_id: Option<String>,
		/// Indication of MCPTT service request.
		#[serde(rename = "mcpttId", default, skip_serializing_if = "Option::is_none")]
		pub mcptt_id: Option<String>,
		/// Indication of MCS service request.
		#[serde(rename = "mcsId", default, skip_serializing_if = "Option::is_none")]
		pub mcs_id: Option<String>,
		/// Contains media component information. The key of the map is the
		/// medCompN attribute.
		#[serde(
			rename = "medComponents",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub med_components: ::std::collections::HashMap<String, MediaComponentRm>,
		#[serde(rename = "mpsAction", default, skip_serializing_if = "Option::is_none")]
		pub mps_action: Option<MpsAction>,
		/// Indication of MPS service request.
		#[serde(rename = "mpsId", default, skip_serializing_if = "Option::is_none")]
		pub mps_id: Option<String>,
		#[serde(
			rename = "preemptControlInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preempt_control_info: Option<PreemptionControlInformationRm>,
		#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
		pub res_prio: Option<ReservPriority>,
		#[serde(
			rename = "servInfStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_inf_status: Option<ServiceInfoStatus>,
		#[serde(
			rename = "sipForkInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sip_fork_ind: Option<SipForkingIndication>,
		#[serde(rename = "sponId", default, skip_serializing_if = "Option::is_none")]
		pub spon_id: Option<SponId>,
		#[serde(
			rename = "sponStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub spon_status: Option<SponsoringStatus>,
		#[serde(
			rename = "tsnBridgeManCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_man_cont: Option<BridgeManagementContainer>,
		#[serde(
			rename = "tsnPortManContDstt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_port_man_cont_dstt: Option<PortManagementContainer>,
		#[serde(
			rename = "tsnPortManContNwtts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
	}

	impl From<&AppSessionContextUpdateData> for AppSessionContextUpdateData {
		fn from(value: &AppSessionContextUpdateData) -> Self {
			value.clone()
		}
	}

	/// Identifies the modifications to an Individual Application Session
	/// Context and/or the modifications to the sub-resource Events
	/// Subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the modifications to an Individual
	/// Application Session Context and/or the modifications to the sub-resource
	/// Events Subscription.",
	///  "type": "object",
	///  "properties": {
	///    "ascReqData": {
	///      "$ref": "#/components/schemas/AppSessionContextUpdateData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppSessionContextUpdateDataPatch {
		#[serde(
			rename = "ascReqData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub asc_req_data: Option<AppSessionContextUpdateData>,
	}

	impl From<&AppSessionContextUpdateDataPatch> for AppSessionContextUpdateDataPatch {
		fn from(value: &AppSessionContextUpdateDataPatch) -> Self {
			value.clone()
		}
	}

	/// ApplicationDescriptor
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
	pub struct ApplicationDescriptor(pub Bytes);

	impl ::std::ops::Deref for ApplicationDescriptor {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<ApplicationDescriptor> for Bytes {
		fn from(value: ApplicationDescriptor) -> Self {
			value.0
		}
	}

	impl From<&ApplicationDescriptor> for ApplicationDescriptor {
		fn from(value: &ApplicationDescriptor) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for ApplicationDescriptor {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ApplicationDescriptor {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ApplicationDescriptor {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ApplicationDescriptor {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ApplicationDescriptor {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ApplicationDescriptor {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Contains an identity of an application service provider.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an identity of an application service
	/// provider.",
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
	pub struct AspId(pub String);

	impl ::std::ops::Deref for AspId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AspId> for String {
		fn from(value: AspId) -> Self {
			value.0
		}
	}

	impl From<&AspId> for AspId {
		fn from(value: &AspId) -> Self {
			value.clone()
		}
	}

	impl From<String> for AspId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AspId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for AspId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the ATSSS capability supported for the MA PDU Session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the ATSSS capability supported for the MA PDU
	/// Session.",
	///  "type": "string",
	///  "enum": [
	///    "MPTCP_ATSSS_LL_WITH_ASMODE_UL",
	///    "MPTCP_ATSSS_LL_WITH_EXSDMODE_DL_ASMODE_UL",
	///    "MPTCP_ATSSS_LL_WITH_ASMODE_DLUL",
	///    "ATSSS_LL",
	///    "MPTCP_ATSSS_LL"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AtsssCapability {
		#[default]
		#[serde(rename = "MPTCP_ATSSS_LL_WITH_ASMODE_UL")]
		MptcpAtsssLlWithAsmodeUl,
		#[serde(rename = "MPTCP_ATSSS_LL_WITH_EXSDMODE_DL_ASMODE_UL")]
		MptcpAtsssLlWithExsdmodeDlAsmodeUl,
		#[serde(rename = "MPTCP_ATSSS_LL_WITH_ASMODE_DLUL")]
		MptcpAtsssLlWithAsmodeDlul,
		#[serde(rename = "ATSSS_LL")]
		AtsssLl,
		#[serde(rename = "MPTCP_ATSSS_LL")]
		MptcpAtsssLl,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl ToString for AtsssCapability {
		fn to_string(&self) -> String {
			match *self {
				Self::MptcpAtsssLlWithAsmodeUl => "MPTCP_ATSSS_LL_WITH_ASMODE_UL".to_string(),
				Self::MptcpAtsssLlWithExsdmodeDlAsmodeUl => {
					"MPTCP_ATSSS_LL_WITH_EXSDMODE_DL_ASMODE_UL".to_string()
				}
				Self::MptcpAtsssLlWithAsmodeDlul => "MPTCP_ATSSS_LL_WITH_ASMODE_DLUL".to_string(),
				Self::AtsssLl => "ATSSS_LL".to_string(),
				Self::MptcpAtsssLl => "MPTCP_ATSSS_LL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AtsssCapability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MPTCP_ATSSS_LL_WITH_ASMODE_UL" => Ok(Self::MptcpAtsssLlWithAsmodeUl),
				"MPTCP_ATSSS_LL_WITH_EXSDMODE_DL_ASMODE_UL" => {
					Ok(Self::MptcpAtsssLlWithExsdmodeDlAsmodeUl)
				}
				"MPTCP_ATSSS_LL_WITH_ASMODE_DLUL" => Ok(Self::MptcpAtsssLlWithAsmodeDlul),
				"ATSSS_LL" => Ok(Self::AtsssLl),
				"MPTCP_ATSSS_LL" => Ok(Self::MptcpAtsssLl),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AtsssCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AtsssCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AtsssCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Represents an Individual BDT policy resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual BDT policy resource.",
	///  "type": "object",
	///  "properties": {
	///    "bdtPolData": {
	///      "$ref": "#/components/schemas/BdtPolicyData"
	///    },
	///    "bdtReqData": {
	///      "$ref": "#/components/schemas/BdtReqData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BdtPolicy {
		#[serde(
			rename = "bdtPolData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub bdt_pol_data: Option<BdtPolicyData>,
		#[serde(
			rename = "bdtReqData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub bdt_req_data: Option<BdtReqData>,
	}

	impl From<&BdtPolicy> for BdtPolicy {
		fn from(value: &BdtPolicy) -> Self {
			value.clone()
		}
	}

	/// Describes the authorization data of an Individual BDT policy resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the authorization data of an Individual BDT
	/// policy resource.",
	///  "type": "object",
	///  "required": [
	///    "bdtRefId",
	///    "transfPolicies"
	///  ],
	///  "properties": {
	///    "bdtRefId": {
	///      "$ref": "#/components/schemas/BdtReferenceId"
	///    },
	///    "selTransPolicyId": {
	///      "description": "Contains an identity of the selected transfer
	/// policy.",
	///      "type": "integer"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "transfPolicies": {
	///      "description": "Contains transfer policies.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransferPolicy"
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
	pub struct BdtPolicyData {
		#[serde(rename = "bdtRefId")]
		pub bdt_ref_id: BdtReferenceId,
		/// Contains an identity of the selected transfer policy.
		#[serde(
			rename = "selTransPolicyId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sel_trans_policy_id: Option<i64>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
		/// Contains transfer policies.
		#[serde(rename = "transfPolicies")]
		pub transf_policies: Vec<TransferPolicy>,
	}

	impl From<&BdtPolicyData> for BdtPolicyData {
		fn from(value: &BdtPolicyData) -> Self {
			value.clone()
		}
	}

	/// A JSON Merge Patch body schema containing modification instruction to be
	/// performed on the bdtPolData attribute of the BdtPolicy data structure to
	/// select a transfer policy. Adds selTransPolicyId to BdtPolicyData data
	/// structure.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A JSON Merge Patch body schema containing modification
	/// instruction to be performed on the bdtPolData attribute of the BdtPolicy
	/// data structure to select a transfer policy. Adds selTransPolicyId to
	/// BdtPolicyData data structure.\n",
	///  "type": "object",
	///  "required": [
	///    "selTransPolicyId"
	///  ],
	///  "properties": {
	///    "selTransPolicyId": {
	///      "description": "Contains an identity (i.e. transPolicyId value) of
	/// the selected transfer policy. If the BdtNotification_5G feature is
	/// supported value 0 indicates that no transfer policy is selected.\n",
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BdtPolicyDataPatch {
		/// Contains an identity (i.e. transPolicyId value) of the selected
		/// transfer policy. If the BdtNotification_5G feature is supported
		/// value 0 indicates that no transfer policy is selected.
		#[serde(rename = "selTransPolicyId")]
		pub sel_trans_policy_id: i64,
	}

	impl From<&BdtPolicyDataPatch> for BdtPolicyDataPatch {
		fn from(value: &BdtPolicyDataPatch) -> Self {
			value.clone()
		}
	}

	/// string identifying a BDT Reference ID as defined in clause 5.3.3 of 3GPP
	/// TS 29.154.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string identifying a BDT Reference ID as defined in
	/// clause 5.3.3 of 3GPP TS 29.154.",
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
	pub struct BdtReferenceId(pub String);

	impl ::std::ops::Deref for BdtReferenceId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<BdtReferenceId> for String {
		fn from(value: BdtReferenceId) -> Self {
			value.0
		}
	}

	impl From<&BdtReferenceId> for BdtReferenceId {
		fn from(value: &BdtReferenceId) -> Self {
			value.clone()
		}
	}

	impl From<String> for BdtReferenceId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for BdtReferenceId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for BdtReferenceId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains service requirements for creation a new Individual BDT policy
	/// resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains service requirements for creation a new
	/// Individual BDT policy resource.\n",
	///  "type": "object",
	///  "required": [
	///    "aspId",
	///    "desTimeInt",
	///    "numOfUes",
	///    "volPerUe"
	///  ],
	///  "properties": {
	///    "aspId": {
	///      "$ref": "#/components/schemas/AspId"
	///    },
	///    "desTimeInt": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "interGroupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "numOfUes": {
	///      "description": "Indicates a number of UEs.",
	///      "type": "integer"
	///    },
	///    "nwAreaInfo": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "trafficDes": {
	///      "$ref": "#/components/schemas/TrafficDescriptor"
	///    },
	///    "volPerUe": {
	///      "$ref": "#/components/schemas/UsageThreshold"
	///    },
	///    "warnNotifReq": {
	///      "description": "Indicates whether the BDT warning notification is
	/// enabled or disabled.",
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
	pub struct BdtReqData {
		#[serde(rename = "aspId")]
		pub asp_id: AspId,
		#[serde(rename = "desTimeInt")]
		pub des_time_int: TimeWindow,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "interGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_group_id: Option<GroupId>,
		#[serde(rename = "notifUri", default, skip_serializing_if = "Option::is_none")]
		pub notif_uri: Option<Uri>,
		/// Indicates a number of UEs.
		#[serde(rename = "numOfUes")]
		pub num_of_ues: i64,
		#[serde(
			rename = "nwAreaInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nw_area_info: Option<NetworkAreaInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
		#[serde(
			rename = "trafficDes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_des: Option<TrafficDescriptor>,
		#[serde(rename = "volPerUe")]
		pub vol_per_ue: UsageThreshold,
		/// Indicates whether the BDT warning notification is enabled or
		/// disabled.
		#[serde(rename = "warnNotifReq", default)]
		pub warn_notif_req: bool,
	}

	impl From<&BdtReqData> for BdtReqData {
		fn from(value: &BdtReqData) -> Self {
			value.clone()
		}
	}

	/// A JSON Merge Patch body schema containing modification instruction to be
	/// performed on the bdtReqData attribute of the BdtPolicy data structure to
	/// indicate whether the BDT warning notification is enabled or disabled.
	/// Modifies warnNotifReq from BdtReqData data structure.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A JSON Merge Patch body schema containing modification
	/// instruction to be performed on the bdtReqData attribute of the BdtPolicy
	/// data structure to indicate whether the BDT warning notification is
	/// enabled or disabled. Modifies warnNotifReq from BdtReqData data
	/// structure.\n",
	///  "type": "object",
	///  "properties": {
	///    "warnNotifReq": {
	///      "description": "Indicates whether the BDT warning notification is
	/// enabled or disabled.",
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BdtReqDataPatch {
		/// Indicates whether the BDT warning notification is enabled or
		/// disabled.
		#[serde(
			rename = "warnNotifReq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub warn_notif_req: Option<bool>,
	}

	impl From<&BdtReqDataPatch> for BdtReqDataPatch {
		fn from(value: &BdtReqDataPatch) -> Self {
			value.clone()
		}
	}

	/// Contains the UMIC.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UMIC.",
	///  "type": "object",
	///  "required": [
	///    "bridgeManCont"
	///  ],
	///  "properties": {
	///    "bridgeManCont": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BridgeManagementContainer {
		#[serde(rename = "bridgeManCont")]
		pub bridge_man_cont: Bytes,
	}

	impl From<&BridgeManagementContainer> for BridgeManagementContainer {
		fn from(value: &BridgeManagementContainer) -> Self {
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

	/// Contains charging related parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains charging related parameters.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "chgId"
	///  ],
	///  "properties": {
	///    "afChargId": {
	///      "$ref": "#/components/schemas/ApplicationChargingId"
	///    },
	///    "afChargingIdentifier": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "appSvcProvId": {
	///      "description": "Indicates the application service provider
	/// identity.",
	///      "type": "string"
	///    },
	///    "chgId": {
	///      "description": "Univocally identifies the charging control policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "meteringMethod": {
	///      "$ref": "#/components/schemas/MeteringMethod"
	///    },
	///    "offline": {
	///      "description": "Indicates the offline charging is applicable to the
	/// PCC rule when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "online": {
	///      "description": "Indicates the online charging is applicable to the
	/// PCC rule when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "ratingGroup": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "reportingLevel": {
	///      "$ref": "#/components/schemas/ReportingLevel"
	///    },
	///    "sdfHandl": {
	///      "description": "Indicates whether the service data flow is allowed
	/// to start while the SMF is waiting for the response to the credit
	/// request.",
	///      "type": "boolean"
	///    },
	///    "serviceId": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sponsorId": {
	///      "description": "Indicates the sponsor identity.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChargingData(pub Option<ChargingDataInner>);

	impl ::std::ops::Deref for ChargingData {
		type Target = Option<ChargingDataInner>;
		fn deref(&self) -> &Option<ChargingDataInner> {
			&self.0
		}
	}

	impl From<ChargingData> for Option<ChargingDataInner> {
		fn from(value: ChargingData) -> Self {
			value.0
		}
	}

	impl From<&ChargingData> for ChargingData {
		fn from(value: &ChargingData) -> Self {
			value.clone()
		}
	}

	impl From<Option<ChargingDataInner>> for ChargingData {
		fn from(value: Option<ChargingDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains charging related parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains charging related parameters.",
	///  "type": "object",
	///  "required": [
	///    "chgId"
	///  ],
	///  "properties": {
	///    "afChargId": {
	///      "$ref": "#/components/schemas/ApplicationChargingId"
	///    },
	///    "afChargingIdentifier": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "appSvcProvId": {
	///      "description": "Indicates the application service provider
	/// identity.",
	///      "type": "string"
	///    },
	///    "chgId": {
	///      "description": "Univocally identifies the charging control policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "meteringMethod": {
	///      "$ref": "#/components/schemas/MeteringMethod"
	///    },
	///    "offline": {
	///      "description": "Indicates the offline charging is applicable to the
	/// PCC rule when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "online": {
	///      "description": "Indicates the online charging is applicable to the
	/// PCC rule when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "ratingGroup": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "reportingLevel": {
	///      "$ref": "#/components/schemas/ReportingLevel"
	///    },
	///    "sdfHandl": {
	///      "description": "Indicates whether the service data flow is allowed
	/// to start while the SMF is waiting for the response to the credit
	/// request.",
	///      "type": "boolean"
	///    },
	///    "serviceId": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sponsorId": {
	///      "description": "Indicates the sponsor identity.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChargingDataInner {
		#[serde(rename = "afChargId", default, skip_serializing_if = "Option::is_none")]
		pub af_charg_id: Option<ApplicationChargingId>,
		#[serde(
			rename = "afChargingIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_charging_identifier: Option<ChargingId>,
		/// Indicates the application service provider identity.
		#[serde(
			rename = "appSvcProvId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub app_svc_prov_id: Option<String>,
		/// Univocally identifies the charging control policy data within a PDU
		/// session.
		#[serde(rename = "chgId")]
		pub chg_id: String,
		#[serde(
			rename = "meteringMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub metering_method: Option<MeteringMethod>,
		/// Indicates the offline charging is applicable to the PCC rule when it
		/// is included and set to true.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub offline: Option<bool>,
		/// Indicates the online charging is applicable to the PCC rule when it
		/// is included and set to true.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub online: Option<bool>,
		#[serde(
			rename = "ratingGroup",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rating_group: Option<Uint32>,
		#[serde(
			rename = "reportingLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_level: Option<ReportingLevel>,
		/// Indicates whether the service data flow is allowed to start while
		/// the SMF is waiting for the response to the credit request.
		#[serde(rename = "sdfHandl", default, skip_serializing_if = "Option::is_none")]
		pub sdf_handl: Option<bool>,
		#[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
		pub service_id: Option<Uint32>,
		/// Indicates the sponsor identity.
		#[serde(rename = "sponsorId", default, skip_serializing_if = "Option::is_none")]
		pub sponsor_id: Option<String>,
	}

	impl From<&ChargingDataInner> for ChargingDataInner {
		fn from(value: &ChargingDataInner) -> Self {
			value.clone()
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

	/// Contains conditions of applicability for a rule.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains conditions of applicability for a rule.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "condId"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "activationTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "condId": {
	///      "description": "Uniquely identifies the condition data within a PDU
	/// session.",
	///      "type": "string"
	///    },
	///    "deactivationTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ConditionData(pub Option<ConditionDataInner>);

	impl ::std::ops::Deref for ConditionData {
		type Target = Option<ConditionDataInner>;
		fn deref(&self) -> &Option<ConditionDataInner> {
			&self.0
		}
	}

	impl From<ConditionData> for Option<ConditionDataInner> {
		fn from(value: ConditionData) -> Self {
			value.0
		}
	}

	impl From<&ConditionData> for ConditionData {
		fn from(value: &ConditionData) -> Self {
			value.clone()
		}
	}

	impl From<Option<ConditionDataInner>> for ConditionData {
		fn from(value: Option<ConditionDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains conditions of applicability for a rule.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains conditions of applicability for a rule.",
	///  "type": "object",
	///  "required": [
	///    "condId"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "activationTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "condId": {
	///      "description": "Uniquely identifies the condition data within a PDU
	/// session.",
	///      "type": "string"
	///    },
	///    "deactivationTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ConditionDataInner {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(
			rename = "activationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub activation_time: Option<DateTimeRm>,
		/// Uniquely identifies the condition data within a PDU session.
		#[serde(rename = "condId")]
		pub cond_id: String,
		#[serde(
			rename = "deactivationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub deactivation_time: Option<DateTimeRm>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
	}

	impl From<&ConditionDataInner> for ConditionDataInner {
		fn from(value: &ConditionDataInner) -> Self {
			value.clone()
		}
	}

	/// Represents the content version of some content.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the content version of some content.",
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ContentVersion(pub i64);

	impl ::std::ops::Deref for ContentVersion {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ContentVersion> for i64 {
		fn from(value: ContentVersion) -> Self {
			value.0
		}
	}

	impl From<&ContentVersion> for ContentVersion {
		fn from(value: &ContentVersion) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ContentVersion {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ContentVersion {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ContentVersion {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ContentVersion {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ContentVersion {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ContentVersion {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates the reason of the credit management session failure.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the reason of the credit management session
	/// failure.",
	///  "type": "string",
	///  "enum": [
	///    "END_USER_SER_DENIED",
	///    "CREDIT_CTRL_NOT_APP",
	///    "AUTH_REJECTED",
	///    "USER_UNKNOWN",
	///    "RATING_FAILED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum CreditManagementStatus {
		#[default]
		#[serde(rename = "END_USER_SER_DENIED")]
		EndUserSerDenied,
		#[serde(rename = "CREDIT_CTRL_NOT_APP")]
		CreditCtrlNotApp,
		#[serde(rename = "AUTH_REJECTED")]
		AuthRejected,
		#[serde(rename = "USER_UNKNOWN")]
		UserUnknown,
		#[serde(rename = "RATING_FAILED")]
		RatingFailed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CreditManagementStatus> for CreditManagementStatus {
		fn from(value: &CreditManagementStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for CreditManagementStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::EndUserSerDenied => "END_USER_SER_DENIED".to_string(),
				Self::CreditCtrlNotApp => "CREDIT_CTRL_NOT_APP".to_string(),
				Self::AuthRejected => "AUTH_REJECTED".to_string(),
				Self::UserUnknown => "USER_UNKNOWN".to_string(),
				Self::RatingFailed => "RATING_FAILED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CreditManagementStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"END_USER_SER_DENIED" => Ok(Self::EndUserSerDenied),
				"CREDIT_CTRL_NOT_APP" => Ok(Self::CreditCtrlNotApp),
				"AUTH_REJECTED" => Ok(Self::AuthRejected),
				"USER_UNKNOWN" => Ok(Self::UserUnknown),
				"RATING_FAILED" => Ok(Self::RatingFailed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CreditManagementStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CreditManagementStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CreditManagementStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains the downlink data notification control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the downlink data notification control
	/// information.",
	///  "type": "object",
	///  "properties": {
	///    "notifCtrlInds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationControlIndication"
	///      },
	///      "minItems": 1
	///    },
	///    "typesOfNotif": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
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
	pub struct DownlinkDataNotificationControl {
		#[serde(
			rename = "notifCtrlInds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub notif_ctrl_inds: Vec<NotificationControlIndication>,
		#[serde(
			rename = "typesOfNotif",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub types_of_notif: Vec<DlDataDeliveryStatus>,
	}

	impl From<&DownlinkDataNotificationControl> for DownlinkDataNotificationControl {
		fn from(value: &DownlinkDataNotificationControl) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the
	/// DownlinkDataNotificationControl data type, but with the nullable:true
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// DownlinkDataNotificationControl data type, but with the nullable:true
	/// property.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "notifCtrlInds": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationControlIndication"
	///      },
	///      "minItems": 1
	///    },
	///    "typesOfNotif": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
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
	pub struct DownlinkDataNotificationControlRm(
		pub Option<DownlinkDataNotificationControlRmInner>,
	);

	impl ::std::ops::Deref for DownlinkDataNotificationControlRm {
		type Target = Option<DownlinkDataNotificationControlRmInner>;
		fn deref(&self) -> &Option<DownlinkDataNotificationControlRmInner> {
			&self.0
		}
	}

	impl From<DownlinkDataNotificationControlRm> for Option<DownlinkDataNotificationControlRmInner> {
		fn from(value: DownlinkDataNotificationControlRm) -> Self {
			value.0
		}
	}

	impl From<&DownlinkDataNotificationControlRm> for DownlinkDataNotificationControlRm {
		fn from(value: &DownlinkDataNotificationControlRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<DownlinkDataNotificationControlRmInner>> for DownlinkDataNotificationControlRm {
		fn from(value: Option<DownlinkDataNotificationControlRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the
	/// DownlinkDataNotificationControl data type, but with the nullable:true
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// DownlinkDataNotificationControl data type, but with the nullable:true
	/// property.",
	///  "type": "object",
	///  "properties": {
	///    "notifCtrlInds": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationControlIndication"
	///      },
	///      "minItems": 1
	///    },
	///    "typesOfNotif": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
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
	pub struct DownlinkDataNotificationControlRmInner {
		#[serde(
			rename = "notifCtrlInds",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_ctrl_inds: Option<Vec<NotificationControlIndication>>,
		#[serde(
			rename = "typesOfNotif",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub types_of_notif: Option<Vec<DlDataDeliveryStatus>>,
	}

	impl From<&DownlinkDataNotificationControlRmInner> for DownlinkDataNotificationControlRmInner {
		fn from(value: &DownlinkDataNotificationControlRmInner) -> Self {
			value.clone()
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

	/// Contains the rule error reports.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the rule error reports.",
	///  "type": "object",
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "invalidPolicyDecs": {
	///      "description": "Indicates the invalid parameters for the reported
	/// type(s) of the failed policy decision and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InvalidParam"
	///      },
	///      "minItems": 1
	///    },
	///    "polDecFailureReports": {
	///      "description": "Used to report failure of the policy decision
	/// and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyDecisionFailureCode"
	///      },
	///      "minItems": 1
	///    },
	///    "ruleReports": {
	///      "description": "Used to report the PCC rule failure.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RuleReport"
	///      },
	///      "minItems": 1
	///    },
	///    "sessRuleReports": {
	///      "description": "Used to report the session rule failure.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SessionRuleReport"
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
	pub struct ErrorReport {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub error: Option<ProblemDetails>,
		/// Indicates the invalid parameters for the reported type(s) of the
		/// failed policy decision and/or condition data.
		#[serde(
			rename = "invalidPolicyDecs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_policy_decs: Vec<InvalidParam>,
		/// Used to report failure of the policy decision and/or condition data.
		#[serde(
			rename = "polDecFailureReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pol_dec_failure_reports: Vec<PolicyDecisionFailureCode>,
		/// Used to report the PCC rule failure.
		#[serde(rename = "ruleReports", default, skip_serializing_if = "Vec::is_empty")]
		pub rule_reports: Vec<RuleReport>,
		/// Used to report the session rule failure.
		#[serde(
			rename = "sessRuleReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sess_rule_reports: Vec<SessionRuleReport>,
	}

	impl From<&ErrorReport> for ErrorReport {
		fn from(value: &ErrorReport) -> Self {
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
	///      "$ref": "#/components/schemas/FlowDescription1"
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
		pub f_desc: Option<FlowDescription1>,
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

	/// Identifies an UL/DL ethernet flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies an UL/DL ethernet flow.",
	///  "type": "object",
	///  "required": [
	///    "flowNumber"
	///  ],
	///  "properties": {
	///    "ethFlows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-EthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "flowNumber": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EthernetFlowInfo {
		#[serde(rename = "ethFlows", default, skip_serializing_if = "Vec::is_empty")]
		pub eth_flows: Vec<SchemasEthFlowDescription>,
		#[serde(rename = "flowNumber")]
		pub flow_number: i64,
	}

	impl From<&EthernetFlowInfo> for EthernetFlowInfo {
		fn from(value: &EthernetFlowInfo) -> Self {
			value.clone()
		}
	}

	/// Describes the notification of a matched event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the notification of a matched event.",
	///  "type": "object",
	///  "required": [
	///    "evNotifs",
	///    "evSubsUri"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "adReports": {
	///      "description": "Includes the detected application report.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AppDetectionReport"
	///      },
	///      "minItems": 1
	///    },
	///    "addAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "anChargAddr": {
	///      "$ref": "#/components/schemas/AccNetChargingAddress"
	///    },
	///    "anChargIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessNetChargingIdentifier"
	///      },
	///      "minItems": 1
	///    },
	///    "anGwAddr": {
	///      "$ref": "#/components/schemas/AnGwAddress"
	///    },
	///    "evNotifs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "evSubsUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "failedResourcAllocReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ResourcesAllocationInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "noNetLocSupp": {
	///      "$ref": "#/components/schemas/NetLocAccessSupport"
	///    },
	///    "outOfCredReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/OutOfCreditInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "qncReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosNotificationControlInfo1"
	///      },
	///      "minItems": 1
	///    },
	///    "qosMonReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosMonitoringReport1"
	///      },
	///      "minItems": 1
	///    },
	///    "ranNasRelCauses": {
	///      "description": "Contains the RAN and/or NAS release cause.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RanNasRelCause"
	///      },
	///      "minItems": 1
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "relAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "satBackhaulCategory": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "succResourcAllocReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ResourcesAllocationInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "ueLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueLocTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "usgRep": {
	///      "$ref": "#/components/schemas/AccumulatedUsage"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventsNotification {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		/// Includes the detected application report.
		#[serde(rename = "adReports", default, skip_serializing_if = "Vec::is_empty")]
		pub ad_reports: Vec<AppDetectionReport>,
		#[serde(
			rename = "addAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "anChargAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_charg_addr: Option<AccNetChargingAddress>,
		#[serde(rename = "anChargIds", default, skip_serializing_if = "Vec::is_empty")]
		pub an_charg_ids: Vec<AccessNetChargingIdentifier>,
		#[serde(rename = "anGwAddr", default, skip_serializing_if = "Option::is_none")]
		pub an_gw_addr: Option<AnGwAddress>,
		#[serde(rename = "evNotifs")]
		pub ev_notifs: Vec<AfEventNotification>,
		#[serde(rename = "evSubsUri")]
		pub ev_subs_uri: Uri,
		#[serde(
			rename = "failedResourcAllocReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_resourc_alloc_reports: Vec<ResourcesAllocationInfo>,
		#[serde(
			rename = "noNetLocSupp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_net_loc_supp: Option<NetLocAccessSupport>,
		#[serde(
			rename = "outOfCredReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub out_of_cred_reports: Vec<OutOfCreditInformation>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnIdNid>,
		#[serde(rename = "qncReports", default, skip_serializing_if = "Vec::is_empty")]
		pub qnc_reports: Vec<QosNotificationControlInfo1>,
		#[serde(
			rename = "qosMonReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_mon_reports: Vec<QosMonitoringReport1>,
		/// Contains the RAN and/or NAS release cause.
		#[serde(
			rename = "ranNasRelCauses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ran_nas_rel_causes: Vec<RanNasRelCause>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "relAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rel_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "satBackhaulCategory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sat_backhaul_category: Option<SatelliteBackhaulCategory>,
		#[serde(
			rename = "succResourcAllocReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub succ_resourc_alloc_reports: Vec<ResourcesAllocationInfo>,
		#[serde(
			rename = "tsnBridgeManCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_man_cont: Option<BridgeManagementContainer>,
		#[serde(
			rename = "tsnPortManContDstt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_port_man_cont_dstt: Option<PortManagementContainer>,
		#[serde(
			rename = "tsnPortManContNwtts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
		#[serde(rename = "ueLoc", default, skip_serializing_if = "Option::is_none")]
		pub ue_loc: Option<UserLocation>,
		#[serde(rename = "ueLocTime", default, skip_serializing_if = "Option::is_none")]
		pub ue_loc_time: Option<DateTime>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(rename = "usgRep", default, skip_serializing_if = "Option::is_none")]
		pub usg_rep: Option<AccumulatedUsage>,
	}

	impl From<&EventsNotification> for EventsNotification {
		fn from(value: &EventsNotification) -> Self {
			value.clone()
		}
	}

	/// Identifies the events the application subscribes to within an Events
	/// Subscription sub-resource data. It may contain the notification of the
	/// already met events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the events the application subscribes to
	/// within an Events Subscription sub-resource data. It may contain the
	/// notification of the already met events.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/EventsSubscReqData"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EventsNotification"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum EventsSubscPutData {
		#[default]
		SubscReqData(EventsSubscReqData),
		Notification(EventsNotification),
	}

	impl From<&EventsSubscPutData> for EventsSubscPutData {
		fn from(value: &EventsSubscPutData) -> Self {
			value.clone()
		}
	}

	impl From<EventsSubscReqData> for EventsSubscPutData {
		fn from(value: EventsSubscReqData) -> Self {
			Self::SubscReqData(value)
		}
	}

	impl From<EventsNotification> for EventsSubscPutData {
		fn from(value: EventsNotification) -> Self {
			Self::Notification(value)
		}
	}

	/// Identifies the events the application subscribes to.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the events the application subscribes to.",
	///  "type": "object",
	///  "required": [
	///    "events"
	///  ],
	///  "properties": {
	///    "afAppIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfAppId"
	///      },
	///      "minItems": 1
	///    },
	///    "directNotifInd": {
	///      "type": "boolean"
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEventSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "notifCorreId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "qosMon": {
	///      "$ref": "#/components/schemas/QosMonitoringInformation"
	///    },
	///    "reqAnis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequiredAccessInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "reqQosMonParams": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedQosMonitoringParameter"
	///      },
	///      "minItems": 1
	///    },
	///    "usgThres": {
	///      "$ref": "#/components/schemas/UsageThreshold"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventsSubscReqData {
		#[serde(rename = "afAppIds", default, skip_serializing_if = "Vec::is_empty")]
		pub af_app_ids: Vec<AfAppId>,
		#[serde(
			rename = "directNotifInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_notif_ind: Option<bool>,
		pub events: Vec<AfEventSubscription>,
		#[serde(
			rename = "notifCorreId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_corre_id: Option<String>,
		#[serde(rename = "notifUri", default, skip_serializing_if = "Option::is_none")]
		pub notif_uri: Option<Uri>,
		#[serde(rename = "qosMon", default, skip_serializing_if = "Option::is_none")]
		pub qos_mon: Option<QosMonitoringInformation>,
		#[serde(rename = "reqAnis", default, skip_serializing_if = "Vec::is_empty")]
		pub req_anis: Vec<RequiredAccessInfo>,
		#[serde(
			rename = "reqQosMonParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub req_qos_mon_params: Vec<RequestedQosMonitoringParameter>,
		#[serde(rename = "usgThres", default, skip_serializing_if = "Option::is_none")]
		pub usg_thres: Option<UsageThreshold>,
	}

	impl From<&EventsSubscReqData> for EventsSubscReqData {
		fn from(value: &EventsSubscReqData) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the EventsSubscReqData data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// EventsSubscReqData data type, but with the OpenAPI nullable property set
	/// to true.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "events"
	///  ],
	///  "properties": {
	///    "directNotifInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEventSubscription"
	///      }
	///    },
	///    "notifCorreId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "qosMon": {
	///      "$ref": "#/components/schemas/QosMonitoringInformationRm"
	///    },
	///    "reqAnis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequiredAccessInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "reqQosMonParams": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedQosMonitoringParameter"
	///      },
	///      "minItems": 1
	///    },
	///    "usgThres": {
	///      "$ref": "#/components/schemas/UsageThresholdRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventsSubscReqDataRm(pub Option<EventsSubscReqDataRmInner>);

	impl ::std::ops::Deref for EventsSubscReqDataRm {
		type Target = Option<EventsSubscReqDataRmInner>;
		fn deref(&self) -> &Option<EventsSubscReqDataRmInner> {
			&self.0
		}
	}

	impl From<EventsSubscReqDataRm> for Option<EventsSubscReqDataRmInner> {
		fn from(value: EventsSubscReqDataRm) -> Self {
			value.0
		}
	}

	impl From<&EventsSubscReqDataRm> for EventsSubscReqDataRm {
		fn from(value: &EventsSubscReqDataRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<EventsSubscReqDataRmInner>> for EventsSubscReqDataRm {
		fn from(value: Option<EventsSubscReqDataRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the EventsSubscReqData data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// EventsSubscReqData data type, but with the OpenAPI nullable property set
	/// to true.",
	///  "type": "object",
	///  "required": [
	///    "events"
	///  ],
	///  "properties": {
	///    "directNotifInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "events": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEventSubscription"
	///      }
	///    },
	///    "notifCorreId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "qosMon": {
	///      "$ref": "#/components/schemas/QosMonitoringInformationRm"
	///    },
	///    "reqAnis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequiredAccessInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "reqQosMonParams": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedQosMonitoringParameter"
	///      },
	///      "minItems": 1
	///    },
	///    "usgThres": {
	///      "$ref": "#/components/schemas/UsageThresholdRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventsSubscReqDataRmInner {
		#[serde(
			rename = "directNotifInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_notif_ind: Option<bool>,
		pub events: Vec<AfEventSubscription>,
		#[serde(
			rename = "notifCorreId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_corre_id: Option<String>,
		#[serde(rename = "notifUri", default, skip_serializing_if = "Option::is_none")]
		pub notif_uri: Option<Uri>,
		#[serde(rename = "qosMon", default, skip_serializing_if = "Option::is_none")]
		pub qos_mon: Option<QosMonitoringInformationRm>,
		#[serde(rename = "reqAnis", default, skip_serializing_if = "Vec::is_empty")]
		pub req_anis: Vec<RequiredAccessInfo>,
		#[serde(
			rename = "reqQosMonParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub req_qos_mon_params: Vec<RequestedQosMonitoringParameter>,
		#[serde(rename = "usgThres", default, skip_serializing_if = "Option::is_none")]
		pub usg_thres: Option<UsageThresholdRm>,
	}

	impl From<&EventsSubscReqDataRmInner> for EventsSubscReqDataRmInner {
		fn from(value: &EventsSubscReqDataRmInner) -> Self {
			value.clone()
		}
	}

	/// Extends ProblemDetails to also include the acceptable service info.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Extends ProblemDetails to also include the acceptable
	/// service info.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    {
	///      "type": "object",
	///      "properties": {
	///        "acceptableServInfo": {
	///          "$ref": "#/components/schemas/AcceptableServiceInfo"
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
	pub struct ExtendedProblemDetails {
		#[serde(
			rename = "acceptableServInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub acceptable_serv_info: Option<AcceptableServiceInfo>,
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

	impl From<&ExtendedProblemDetails> for ExtendedProblemDetails {
		fn from(value: &ExtendedProblemDetails) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - UNSPECIFIED: Indicates the PCF received the UE sent UE policy delivery
	///   service cause #111
	///  (Protocol error, unspecified).
	/// - UE_NOT_REACHABLE: Indicates the PCF received the notification from the
	///   AMF that the UE is
	///  not reachable.
	/// - UNKNOWN: Indicates unknown reasons upon no response from the UE, e.g.
	///   UPDS message type is
	///  not defined or not implemented by the UE, or not compatible with the
	/// UPDS state, in which  the UE shall ignore the UPDS message.
	/// - UE_TEMP_UNREACHABLE: Indicates the PCF received the notification from
	///   the AMF that the UE
	///  is not reachable but the PCF will retry again.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNSPECIFIED: Indicates the PCF
	/// received the UE sent UE policy delivery service cause #111\n  (Protocol
	/// error, unspecified).\n- UE_NOT_REACHABLE: Indicates the PCF received the
	/// notification from the AMF that the UE is\n  not reachable.\n- UNKNOWN:
	/// Indicates unknown reasons upon no response from the UE, e.g. UPDS
	/// message type is\n  not defined or not implemented by the UE, or not
	/// compatible with the UPDS state, in which\n  the UE shall ignore the UPDS
	/// message.\n- UE_TEMP_UNREACHABLE: Indicates the PCF received the
	/// notification from the AMF that the UE\n  is not reachable but the PCF
	/// will retry again.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNSPECIFIED",
	///    "UE_NOT_REACHABLE",
	///    "UNKNOWN",
	///    "UE_TEMP_UNREACHABLE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum Failure {
		#[default]
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(rename = "UE_NOT_REACHABLE")]
		UeNotReachable,
		#[serde(rename = "UNKNOWN")]
		Unknown,
		#[serde(rename = "UE_TEMP_UNREACHABLE")]
		UeTempUnreachable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Failure> for Failure {
		fn from(value: &Failure) -> Self {
			value.clone()
		}
	}

	impl ToString for Failure {
		fn to_string(&self) -> String {
			match *self {
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UeNotReachable => "UE_NOT_REACHABLE".to_string(),
				Self::Unknown => "UNKNOWN".to_string(),
				Self::UeTempUnreachable => "UE_TEMP_UNREACHABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Failure {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNSPECIFIED" => Ok(Self::Unspecified),
				"UE_NOT_REACHABLE" => Ok(Self::UeNotReachable),
				"UNKNOWN" => Ok(Self::Unknown),
				"UE_TEMP_UNREACHABLE" => Ok(Self::UeTempUnreachable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Failure {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Failure {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Failure {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the cause of the failure in a Partial Success Report.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the cause of the failure in a Partial Success
	/// Report.",
	///  "type": "string",
	///  "enum": [
	///    "PCC_RULE_EVENT",
	///    "PCC_QOS_FLOW_EVENT",
	///    "RULE_PERMANENT_ERROR",
	///    "RULE_TEMPORARY_ERROR",
	///    "POL_DEC_ERROR"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum FailureCause {
		#[default]
		#[serde(rename = "PCC_RULE_EVENT")]
		PccRuleEvent,
		#[serde(rename = "PCC_QOS_FLOW_EVENT")]
		PccQosFlowEvent,
		#[serde(rename = "RULE_PERMANENT_ERROR")]
		RulePermanentError,
		#[serde(rename = "RULE_TEMPORARY_ERROR")]
		RuleTemporaryError,
		#[serde(rename = "POL_DEC_ERROR")]
		PolDecError,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FailureCause> for FailureCause {
		fn from(value: &FailureCause) -> Self {
			value.clone()
		}
	}

	impl ToString for FailureCause {
		fn to_string(&self) -> String {
			match *self {
				Self::PccRuleEvent => "PCC_RULE_EVENT".to_string(),
				Self::PccQosFlowEvent => "PCC_QOS_FLOW_EVENT".to_string(),
				Self::RulePermanentError => "RULE_PERMANENT_ERROR".to_string(),
				Self::RuleTemporaryError => "RULE_TEMPORARY_ERROR".to_string(),
				Self::PolDecError => "POL_DEC_ERROR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FailureCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PCC_RULE_EVENT" => Ok(Self::PccRuleEvent),
				"PCC_QOS_FLOW_EVENT" => Ok(Self::PccQosFlowEvent),
				"RULE_PERMANENT_ERROR" => Ok(Self::RulePermanentError),
				"RULE_TEMPORARY_ERROR" => Ok(Self::RuleTemporaryError),
				"POL_DEC_ERROR" => Ok(Self::PolDecError),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FailureCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FailureCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FailureCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are
	/// - UNK_RULE_ID: Indicates that the pre-provisioned PCC rule could not be
	///   successfully activated because the PCC rule identifier is unknown to
	///   the SMF.
	/// - RA_GR_ERR: Indicate that the PCC rule could not be successfully
	///   installed or enforced because the Rating Group specified within the
	///   Charging Data policy decision which the PCC rule refers to is unknown
	///   or, invalid.
	/// - SER_ID_ERR: Indicate that the PCC rule could not be successfully
	///   installed or enforced because the Service Identifier specified within
	///   the Charging Data policy decision which the PCC rule refers to is
	///   invalid, unknown, or not applicable to the service being charged.
	/// - NF_MAL: Indicate that the PCC rule could not be successfully installed
	///   (for those provisioned from the PCF) or activated (for those
	///   pre-defined in SMF) or enforced (for those already successfully
	///   installed) due to SMF/UPF malfunction.
	/// - RES_LIM: Indicate that the PCC rule could not be successfully
	///   installed (for those provisioned from PCF) or activated (for those
	///   pre-defined in SMF) or enforced (for those already successfully
	///   installed) due to a limitation of resources at the SMF/UPF.
	/// - MAX_NR_QoS_FLOW: Indicate that the PCC rule could not be successfully
	///   installed (for those provisioned from PCF) or activated (for those
	///   pre-defined in SMF) or enforced (for those already successfully
	///   installed) due to the fact that the maximum number of QoS flows has
	///   been reached for the PDU session.
	/// - MISS_FLOW_INFO: Indicate that the PCC rule could not be successfully
	///   installed or enforced because neither the "flowInfos" attribute nor
	///   the "appId" attribute is specified within the PccRule data structure
	///   by the PCF during the first install request of the PCC rule.
	/// - RES_ALLO_FAIL: Indicate that the PCC rule could not be successfully
	///   installed or maintained since the QoS flow establishment/modification
	///   failed, or the QoS flow was released.
	/// - UNSUCC_QOS_VAL: indicate that the QoS validation has failed or when
	///   Guaranteed Bandwidth > Max-Requested-Bandwidth.
	/// - INCOR_FLOW_INFO: Indicate that the PCC rule could not be successfully
	///   installed or modified at the SMF because the provided flow information
	///   is not supported by the network (e.g. the provided IP address(es) or
	///   Ipv6 prefix(es) do not correspond to an IP version applicable for the
	///   PDU session).
	/// - PS_TO_CS_HAN: Indicate that the PCC rule could not be maintained
	///   because of PS to CS handover.
	/// - APP_ID_ERR: Indicate that the rule could not be successfully installed
	///   or enforced because the Application Identifier is invalid, unknown, or
	///   not applicable to the application required for detection.
	/// - NO_QOS_FLOW_BOUND: Indicate that there is no QoS flow which the SMF
	///   can bind the PCC rule(s) to.
	/// - FILTER_RES: Indicate that the Flow Information within the "flowInfos"
	///   attribute cannot be handled by the SMF because any of the restrictions
	///   defined in clause 5.4.2 of 3GPP TS 29.212 was not met.
	/// - MISS_REDI_SER_ADDR: Indicate that the PCC rule could not be
	///   successfully installed or enforced at the SMF because there is no
	///   valid Redirect Server Address within the Traffic Control Data policy
	///   decision which the PCC rule refers to provided by the PCF and no
	///   preconfigured redirection address for this PCC rule at the SMF.
	/// - CM_END_USER_SER_DENIED: Indicate that the charging system denied the
	///   service request due to service restrictions (e.g. terminate rating
	///   group) or limitations related to the end-user, for example the
	///   end-user's account could not cover the requested service.
	/// - CM_CREDIT_CON_NOT_APP: Indicate that the charging system determined
	///   that the service can be granted to the end user but no further credit
	///   control is needed for the service (e.g. service is free of charge or
	///   is treated for offline charging).
	///  - CM_AUTH_REJ: Indicate that the charging system denied the service
	///    request in order to terminate the service for which credit is
	///    requested.
	/// - CM_USER_UNK: Indicate that the specified end user could not be found
	///   in the charging system.
	/// - CM_RAT_FAILED: Indicate that the charging system cannot rate the
	///   service request due to insufficient rating input, incorrect AVP
	///   combination or due to an attribute or an attribute value that is not
	///   recognized or supported in the rating.
	/// - UE_STA_SUSP: Indicates that the UE is in suspend state.
	/// - UNKNOWN_REF_ID: Indicates that the PCC rule could not be successfully
	///   installed/modified because the referenced identifier to a Policy
	///   Decision Data or to a Condition Data is unknown to the SMF.
	/// - INCORRECT_COND_DATA: Indicates that the PCC rule could not be
	///   successfully installed/modified because the referenced Condition data
	///   are incorrect.
	/// - REF_ID_COLLISION: Indicates that PCC rule could not be successfully
	///   installed/modified because the same Policy Decision is referenced by a
	///   session rule (e.g. the session rule and the PCC rule refer to the same
	///   Usage Monitoring decision data).
	/// - TRAFFIC_STEERING_ERROR: Indicates that enforcement of the steering of
	///   traffic to the N6-LAN or 5G-LAN failed; or the dynamic PCC rule could
	///   not be successfully installed or modified at the NF service consumer
	///   because there are invalid traffic steering policy identifier(s) within
	///   the provided Traffic Control Data policy decision to which the PCC
	///   rule refers.
	/// - DNAI_STEERING_ERROR: Indicates that the enforcement of the steering of
	///   traffic to the indicated DNAI failed; or the dynamic PCC rule could
	///   not be successfully installed or modified at the NF service consumer
	///   because there is invalid route information for a DNAI(s) (e.g. routing
	///   profile id is not configured) within the provided Traffic Control Data
	///   policy decision to which the PCC rule refers.
	/// - AN_GW_FAILED: This value is used to indicate that the AN-Gateway has
	///   failed and that the PCF should refrain from sending policy decisions
	///   to the SMF until it is informed that the S-GW has been recovered. This
	///   value shall not be used if the SM Policy association modification
	///   procedure is initiated for PCC rule removal only.
	/// - MAX_NR_PACKET_FILTERS_EXCEEDED: This value is used to indicate that
	///   the PCC rule could not be successfully installed, modified or enforced
	///   at the NF service consumer because the number of supported packet
	///   filters for signalled QoS rules for the PDU session has been reached.
	/// - PACKET_FILTER_TFT_ALLOCATION_EXCEEDED: This value is used to indicate
	///   that the PCC rule is removed at 5GS to EPS mobility because TFT
	///   allocation was not possible since the number of active packet filters
	///   in the EPC bearer is exceeded.
	/// - MUTE_CHG_NOT_ALLOWED: Indicates that the PCC rule could not be
	///   successfully modified because the mute condition for application
	///   detection report cannot be changed. Applicable when the functionality
	///   introduced with the ADC feature applies.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- UNK_RULE_ID: Indicates that the
	/// pre-provisioned PCC rule could not be successfully activated because the
	/// PCC rule identifier is unknown to the SMF.\n- RA_GR_ERR: Indicate that
	/// the PCC rule could not be successfully installed or enforced because the
	/// Rating Group specified within the Charging Data policy decision which
	/// the PCC rule refers to is unknown or, invalid.\n- SER_ID_ERR: Indicate
	/// that the PCC rule could not be successfully installed or enforced
	/// because the Service Identifier specified within the Charging Data policy
	/// decision which the PCC rule refers to is invalid, unknown, or not
	/// applicable to the service being charged.\n- NF_MAL: Indicate that the
	/// PCC rule could not be successfully installed (for those provisioned from
	/// the PCF) or activated (for those pre-defined in SMF) or enforced (for
	/// those already successfully installed) due to SMF/UPF malfunction.\n-
	/// RES_LIM: Indicate that the PCC rule could not be successfully installed
	/// (for those provisioned from PCF) or activated (for those pre-defined in
	/// SMF) or enforced (for those already successfully installed) due to a
	/// limitation of resources at the SMF/UPF.\n- MAX_NR_QoS_FLOW: Indicate
	/// that the PCC rule could not be successfully installed (for those
	/// provisioned from PCF) or activated (for those pre-defined in SMF) or
	/// enforced (for those already successfully installed) due to the fact that
	/// the maximum number of QoS flows has been reached for the PDU session.\n-
	/// MISS_FLOW_INFO: Indicate that the PCC rule could not be successfully
	/// installed or enforced because neither the \"flowInfos\" attribute nor
	/// the \"appId\" attribute is specified within the PccRule data structure
	/// by the PCF during the first install request of the PCC rule.\n-
	/// RES_ALLO_FAIL: Indicate that the PCC rule could not be successfully
	/// installed or maintained since the QoS flow establishment/modification
	/// failed, or the QoS flow was released.\n- UNSUCC_QOS_VAL: indicate that
	/// the QoS validation has failed or when Guaranteed Bandwidth >
	/// Max-Requested-Bandwidth.\n- INCOR_FLOW_INFO: Indicate that the PCC rule
	/// could not be successfully installed or modified at the SMF because the
	/// provided flow information is not supported by the network (e.g. the
	/// provided IP address(es) or Ipv6 prefix(es) do not correspond to an IP
	/// version applicable for the PDU session).\n- PS_TO_CS_HAN: Indicate that
	/// the PCC rule could not be maintained because of PS to CS handover.\n-
	/// APP_ID_ERR: Indicate that the rule could not be successfully installed
	/// or enforced because the Application Identifier is invalid, unknown, or
	/// not applicable to the application required for detection.\n-
	/// NO_QOS_FLOW_BOUND: Indicate that there is no QoS flow which the SMF can
	/// bind the PCC rule(s) to.\n- FILTER_RES: Indicate that the Flow
	/// Information within the \"flowInfos\" attribute cannot be handled by the
	/// SMF because any of the restrictions defined in clause 5.4.2 of 3GPP TS
	/// 29.212 was not met.\n- MISS_REDI_SER_ADDR: Indicate that the PCC rule
	/// could not be successfully installed or enforced at the SMF because there
	/// is no valid Redirect Server Address within the Traffic Control Data
	/// policy decision which the PCC rule refers to provided by the PCF and no
	/// preconfigured redirection address for this PCC rule at the SMF.\n-
	/// CM_END_USER_SER_DENIED: Indicate that the charging system denied the
	/// service request due to service restrictions (e.g. terminate rating
	/// group) or limitations related to the end-user, for example the
	/// end-user's account could not cover the requested service.\n-
	/// CM_CREDIT_CON_NOT_APP: Indicate that the charging system determined that
	/// the service can be granted to the end user but no further credit control
	/// is needed for the service (e.g. service is free of charge or is treated
	/// for offline charging).\n  - CM_AUTH_REJ: Indicate that the charging
	/// system denied the service request in order to terminate the service for
	/// which credit is requested.\n- CM_USER_UNK: Indicate that the specified
	/// end user could not be found in the charging system.\n- CM_RAT_FAILED:
	/// Indicate that the charging system cannot rate the service request due to
	/// insufficient rating input, incorrect AVP combination or due to an
	/// attribute or an attribute value that is not recognized or supported in
	/// the rating.\n- UE_STA_SUSP: Indicates that the UE is in suspend
	/// state.\n- UNKNOWN_REF_ID: Indicates that the PCC rule could not be
	/// successfully installed/modified because the referenced identifier to a
	/// Policy Decision Data or to a Condition Data is unknown to the SMF.\n-
	/// INCORRECT_COND_DATA: Indicates that the PCC rule could not be
	/// successfully installed/modified because the referenced Condition data
	/// are incorrect.\n- REF_ID_COLLISION: Indicates that PCC rule could not be
	/// successfully installed/modified because the same Policy Decision is
	/// referenced by a session rule (e.g. the session rule and the PCC rule
	/// refer to the same Usage Monitoring decision data).\n-
	/// TRAFFIC_STEERING_ERROR: Indicates that enforcement of the steering of
	/// traffic to the N6-LAN or 5G-LAN failed; or the dynamic PCC rule could
	/// not be successfully installed or modified at the NF service consumer
	/// because there are invalid traffic steering policy identifier(s) within
	/// the provided Traffic Control Data policy decision to which the PCC rule
	/// refers.\n- DNAI_STEERING_ERROR: Indicates that the enforcement of the
	/// steering of traffic to the indicated DNAI failed; or the dynamic PCC
	/// rule could not be successfully installed or modified at the NF service
	/// consumer because there is invalid route information for a DNAI(s) (e.g.
	/// routing profile id is not configured) within the provided Traffic
	/// Control Data policy decision to which the PCC rule refers.\n-
	/// AN_GW_FAILED: This value is used to indicate that the AN-Gateway has
	/// failed and that the PCF should refrain from sending policy decisions to
	/// the SMF until it is informed that the S-GW has been recovered. This
	/// value shall not be used if the SM Policy association modification
	/// procedure is initiated for PCC rule removal only.\n-
	/// MAX_NR_PACKET_FILTERS_EXCEEDED: This value is used to indicate that the
	/// PCC rule could not be successfully installed, modified or enforced at
	/// the NF service consumer because the number of supported packet filters
	/// for signalled QoS rules for the PDU session has been reached.\n-
	/// PACKET_FILTER_TFT_ALLOCATION_EXCEEDED: This value is used to indicate
	/// that the PCC rule is removed at 5GS to EPS mobility because TFT
	/// allocation was not possible since the number of active packet filters in
	/// the EPC bearer is exceeded.\n- MUTE_CHG_NOT_ALLOWED: Indicates that the
	/// PCC rule could not be successfully modified because the mute condition
	/// for application detection report cannot be changed. Applicable when the
	/// functionality introduced with the ADC feature applies.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNK_RULE_ID",
	///    "RA_GR_ERR",
	///    "SER_ID_ERR",
	///    "NF_MAL",
	///    "RES_LIM",
	///    "MAX_NR_QoS_FLOW",
	///    "MISS_FLOW_INFO",
	///    "RES_ALLO_FAIL",
	///    "UNSUCC_QOS_VAL",
	///    "INCOR_FLOW_INFO",
	///    "PS_TO_CS_HAN",
	///    "APP_ID_ERR",
	///    "NO_QOS_FLOW_BOUND",
	///    "FILTER_RES",
	///    "MISS_REDI_SER_ADDR",
	///    "CM_END_USER_SER_DENIED",
	///    "CM_CREDIT_CON_NOT_APP",
	///    "CM_AUTH_REJ",
	///    "CM_USER_UNK",
	///    "CM_RAT_FAILED",
	///    "UE_STA_SUSP",
	///    "UNKNOWN_REF_ID",
	///    "INCORRECT_COND_DATA",
	///    "REF_ID_COLLISION",
	///    "TRAFFIC_STEERING_ERROR",
	///    "DNAI_STEERING_ERROR",
	///    "AN_GW_FAILE",
	///    "MAX_NR_PACKET_FILTERS_EXCEEDED",
	///    "PACKET_FILTER_TFT_ALLOCATION_EXCEEDED",
	///    "MUTE_CHG_NOT_ALLOWED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum FailureCode {
		#[default]
		#[serde(rename = "UNK_RULE_ID")]
		UnkRuleId,
		#[serde(rename = "RA_GR_ERR")]
		RaGrErr,
		#[serde(rename = "SER_ID_ERR")]
		SerIdErr,
		#[serde(rename = "NF_MAL")]
		NfMal,
		#[serde(rename = "RES_LIM")]
		ResLim,
		#[serde(rename = "MAX_NR_QoS_FLOW")]
		MaxNrQoSFlow,
		#[serde(rename = "MISS_FLOW_INFO")]
		MissFlowInfo,
		#[serde(rename = "RES_ALLO_FAIL")]
		ResAlloFail,
		#[serde(rename = "UNSUCC_QOS_VAL")]
		UnsuccQosVal,
		#[serde(rename = "INCOR_FLOW_INFO")]
		IncorFlowInfo,
		#[serde(rename = "PS_TO_CS_HAN")]
		PsToCsHan,
		#[serde(rename = "APP_ID_ERR")]
		AppIdErr,
		#[serde(rename = "NO_QOS_FLOW_BOUND")]
		NoQosFlowBound,
		#[serde(rename = "FILTER_RES")]
		FilterRes,
		#[serde(rename = "MISS_REDI_SER_ADDR")]
		MissRediSerAddr,
		#[serde(rename = "CM_END_USER_SER_DENIED")]
		CmEndUserSerDenied,
		#[serde(rename = "CM_CREDIT_CON_NOT_APP")]
		CmCreditConNotApp,
		#[serde(rename = "CM_AUTH_REJ")]
		CmAuthRej,
		#[serde(rename = "CM_USER_UNK")]
		CmUserUnk,
		#[serde(rename = "CM_RAT_FAILED")]
		CmRatFailed,
		#[serde(rename = "UE_STA_SUSP")]
		UeStaSusp,
		#[serde(rename = "UNKNOWN_REF_ID")]
		UnknownRefId,
		#[serde(rename = "INCORRECT_COND_DATA")]
		IncorrectCondData,
		#[serde(rename = "REF_ID_COLLISION")]
		RefIdCollision,
		#[serde(rename = "TRAFFIC_STEERING_ERROR")]
		TrafficSteeringError,
		#[serde(rename = "DNAI_STEERING_ERROR")]
		DnaiSteeringError,
		#[serde(rename = "AN_GW_FAILE")]
		AnGwFaile,
		#[serde(rename = "MAX_NR_PACKET_FILTERS_EXCEEDED")]
		MaxNrPacketFiltersExceeded,
		#[serde(rename = "PACKET_FILTER_TFT_ALLOCATION_EXCEEDED")]
		PacketFilterTftAllocationExceeded,
		#[serde(rename = "MUTE_CHG_NOT_ALLOWED")]
		MuteChgNotAllowed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FailureCode> for FailureCode {
		fn from(value: &FailureCode) -> Self {
			value.clone()
		}
	}

	impl ToString for FailureCode {
		fn to_string(&self) -> String {
			match *self {
				Self::UnkRuleId => "UNK_RULE_ID".to_string(),
				Self::RaGrErr => "RA_GR_ERR".to_string(),
				Self::SerIdErr => "SER_ID_ERR".to_string(),
				Self::NfMal => "NF_MAL".to_string(),
				Self::ResLim => "RES_LIM".to_string(),
				Self::MaxNrQoSFlow => "MAX_NR_QoS_FLOW".to_string(),
				Self::MissFlowInfo => "MISS_FLOW_INFO".to_string(),
				Self::ResAlloFail => "RES_ALLO_FAIL".to_string(),
				Self::UnsuccQosVal => "UNSUCC_QOS_VAL".to_string(),
				Self::IncorFlowInfo => "INCOR_FLOW_INFO".to_string(),
				Self::PsToCsHan => "PS_TO_CS_HAN".to_string(),
				Self::AppIdErr => "APP_ID_ERR".to_string(),
				Self::NoQosFlowBound => "NO_QOS_FLOW_BOUND".to_string(),
				Self::FilterRes => "FILTER_RES".to_string(),
				Self::MissRediSerAddr => "MISS_REDI_SER_ADDR".to_string(),
				Self::CmEndUserSerDenied => "CM_END_USER_SER_DENIED".to_string(),
				Self::CmCreditConNotApp => "CM_CREDIT_CON_NOT_APP".to_string(),
				Self::CmAuthRej => "CM_AUTH_REJ".to_string(),
				Self::CmUserUnk => "CM_USER_UNK".to_string(),
				Self::CmRatFailed => "CM_RAT_FAILED".to_string(),
				Self::UeStaSusp => "UE_STA_SUSP".to_string(),
				Self::UnknownRefId => "UNKNOWN_REF_ID".to_string(),
				Self::IncorrectCondData => "INCORRECT_COND_DATA".to_string(),
				Self::RefIdCollision => "REF_ID_COLLISION".to_string(),
				Self::TrafficSteeringError => "TRAFFIC_STEERING_ERROR".to_string(),
				Self::DnaiSteeringError => "DNAI_STEERING_ERROR".to_string(),
				Self::AnGwFaile => "AN_GW_FAILE".to_string(),
				Self::MaxNrPacketFiltersExceeded => "MAX_NR_PACKET_FILTERS_EXCEEDED".to_string(),
				Self::PacketFilterTftAllocationExceeded => {
					"PACKET_FILTER_TFT_ALLOCATION_EXCEEDED".to_string()
				}
				Self::MuteChgNotAllowed => "MUTE_CHG_NOT_ALLOWED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FailureCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNK_RULE_ID" => Ok(Self::UnkRuleId),
				"RA_GR_ERR" => Ok(Self::RaGrErr),
				"SER_ID_ERR" => Ok(Self::SerIdErr),
				"NF_MAL" => Ok(Self::NfMal),
				"RES_LIM" => Ok(Self::ResLim),
				"MAX_NR_QoS_FLOW" => Ok(Self::MaxNrQoSFlow),
				"MISS_FLOW_INFO" => Ok(Self::MissFlowInfo),
				"RES_ALLO_FAIL" => Ok(Self::ResAlloFail),
				"UNSUCC_QOS_VAL" => Ok(Self::UnsuccQosVal),
				"INCOR_FLOW_INFO" => Ok(Self::IncorFlowInfo),
				"PS_TO_CS_HAN" => Ok(Self::PsToCsHan),
				"APP_ID_ERR" => Ok(Self::AppIdErr),
				"NO_QOS_FLOW_BOUND" => Ok(Self::NoQosFlowBound),
				"FILTER_RES" => Ok(Self::FilterRes),
				"MISS_REDI_SER_ADDR" => Ok(Self::MissRediSerAddr),
				"CM_END_USER_SER_DENIED" => Ok(Self::CmEndUserSerDenied),
				"CM_CREDIT_CON_NOT_APP" => Ok(Self::CmCreditConNotApp),
				"CM_AUTH_REJ" => Ok(Self::CmAuthRej),
				"CM_USER_UNK" => Ok(Self::CmUserUnk),
				"CM_RAT_FAILED" => Ok(Self::CmRatFailed),
				"UE_STA_SUSP" => Ok(Self::UeStaSusp),
				"UNKNOWN_REF_ID" => Ok(Self::UnknownRefId),
				"INCORRECT_COND_DATA" => Ok(Self::IncorrectCondData),
				"REF_ID_COLLISION" => Ok(Self::RefIdCollision),
				"TRAFFIC_STEERING_ERROR" => Ok(Self::TrafficSteeringError),
				"DNAI_STEERING_ERROR" => Ok(Self::DnaiSteeringError),
				"AN_GW_FAILE" => Ok(Self::AnGwFaile),
				"MAX_NR_PACKET_FILTERS_EXCEEDED" => Ok(Self::MaxNrPacketFiltersExceeded),
				"PACKET_FILTER_TFT_ALLOCATION_EXCEEDED" => {
					Ok(Self::PacketFilterTftAllocationExceeded)
				}
				"MUTE_CHG_NOT_ALLOWED" => Ok(Self::MuteChgNotAllowed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FailureCode {
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

	/// Defines a packet filter for an IP flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Defines a packet filter for an IP flow.",
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
	pub struct FlowDescription1(pub String);

	impl ::std::ops::Deref for FlowDescription1 {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<FlowDescription1> for String {
		fn from(value: FlowDescription1) -> Self {
			value.0
		}
	}

	impl From<&FlowDescription1> for FlowDescription1 {
		fn from(value: &FlowDescription1) -> Self {
			value.clone()
		}
	}

	impl From<String> for FlowDescription1 {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for FlowDescription1 {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for FlowDescription1 {
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

	/// This data type is defined in the same way as the "FlowDirection" data
	/// type, with the only difference that it allows null value.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// \"FlowDirection\" data type, with the only difference that it allows
	/// null value.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/FlowDirection"
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
	pub enum FlowDirectionRm {
		#[default]
		FlowDirection(FlowDirection),
		NullValue(NullValue),
	}

	impl From<&FlowDirectionRm> for FlowDirectionRm {
		fn from(value: &FlowDirectionRm) -> Self {
			value.clone()
		}
	}

	impl From<FlowDirection> for FlowDirectionRm {
		fn from(value: FlowDirection) -> Self {
			Self::FlowDirection(value)
		}
	}

	impl From<NullValue> for FlowDirectionRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Contains the flow information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the flow information.",
	///  "type": "object",
	///  "properties": {
	///    "ethFlowDescription": {
	///      "$ref": "#/components/schemas/schemas-EthFlowDescription"
	///    },
	///    "flowDescription": {
	///      "$ref": "#/components/schemas/FlowDescription"
	///    },
	///    "flowDirection": {
	///      "$ref": "#/components/schemas/FlowDirectionRm"
	///    },
	///    "flowLabel": {
	///      "description": "the Ipv6 flow label header field.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "packFiltId": {
	///      "description": "An identifier of packet filter.",
	///      "type": "string"
	///    },
	///    "packetFilterUsage": {
	///      "description": "The packet shall be sent to the UE.",
	///      "type": "boolean"
	///    },
	///    "spi": {
	///      "description": "the security parameter index of the IPSec packet.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "tosTrafficClass": {
	///      "description": "Contains the Ipv4 Type-of-Service and mask field or
	/// the Ipv6 Traffic-Class field and mask field.",
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
	pub struct FlowInformation {
		#[serde(
			rename = "ethFlowDescription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eth_flow_description: Option<SchemasEthFlowDescription>,
		#[serde(
			rename = "flowDescription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub flow_description: Option<FlowDescription>,
		#[serde(
			rename = "flowDirection",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub flow_direction: Option<FlowDirectionRm>,
		/// the Ipv6 flow label header field.
		#[serde(rename = "flowLabel", default, skip_serializing_if = "Option::is_none")]
		pub flow_label: Option<String>,
		/// An identifier of packet filter.
		#[serde(
			rename = "packFiltId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pack_filt_id: Option<String>,
		/// The packet shall be sent to the UE.
		#[serde(
			rename = "packetFilterUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub packet_filter_usage: Option<bool>,
		/// the security parameter index of the IPSec packet.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub spi: Option<String>,
		/// Contains the Ipv4 Type-of-Service and mask field or the Ipv6
		/// Traffic-Class field and mask field.
		#[serde(
			rename = "tosTrafficClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tos_traffic_class: Option<String>,
	}

	impl From<&FlowInformation> for FlowInformation {
		fn from(value: &FlowInformation) -> Self {
			value.clone()
		}
	}

	/// Describes whether the IP flow(s) are enabled or disabled.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes whether the IP flow(s) are enabled or
	/// disabled.",
	///  "type": "string",
	///  "enum": [
	///    "ENABLED-UPLINK",
	///    "ENABLED-DOWNLINK",
	///    "ENABLED",
	///    "DISABLED",
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
	pub enum FlowStatus {
		#[default]
		#[serde(rename = "ENABLED-UPLINK")]
		EnabledUplink,
		#[serde(rename = "ENABLED-DOWNLINK")]
		EnabledDownlink,
		#[serde(rename = "ENABLED")]
		Enabled,
		#[serde(rename = "DISABLED")]
		Disabled,
		#[serde(rename = "REMOVED")]
		Removed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FlowStatus> for FlowStatus {
		fn from(value: &FlowStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for FlowStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::EnabledUplink => "ENABLED-UPLINK".to_string(),
				Self::EnabledDownlink => "ENABLED-DOWNLINK".to_string(),
				Self::Enabled => "ENABLED".to_string(),
				Self::Disabled => "DISABLED".to_string(),
				Self::Removed => "REMOVED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FlowStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ENABLED-UPLINK" => Ok(Self::EnabledUplink),
				"ENABLED-DOWNLINK" => Ok(Self::EnabledDownlink),
				"ENABLED" => Ok(Self::Enabled),
				"DISABLED" => Ok(Self::Disabled),
				"REMOVED" => Ok(Self::Removed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FlowStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FlowStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FlowStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the flow usage of the flows described by a media subcomponent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the flow usage of the flows described by a
	/// media subcomponent.",
	///  "type": "string",
	///  "enum": [
	///    "NO_INFO",
	///    "RTCP",
	///    "AF_SIGNALLING"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum FlowUsage {
		#[default]
		#[serde(rename = "NO_INFO")]
		NoInfo,
		#[serde(rename = "RTCP")]
		Rtcp,
		#[serde(rename = "AF_SIGNALLING")]
		AfSignalling,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FlowUsage> for FlowUsage {
		fn from(value: &FlowUsage) -> Self {
			value.clone()
		}
	}

	impl ToString for FlowUsage {
		fn to_string(&self) -> String {
			match *self {
				Self::NoInfo => "NO_INFO".to_string(),
				Self::Rtcp => "RTCP".to_string(),
				Self::AfSignalling => "AF_SIGNALLING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FlowUsage {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NO_INFO" => Ok(Self::NoInfo),
				"RTCP" => Ok(Self::Rtcp),
				"AF_SIGNALLING" => Ok(Self::AfSignalling),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Identifies the flows.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the flows.",
	///  "type": "object",
	///  "required": [
	///    "medCompN"
	///  ],
	///  "properties": {
	///    "contVers": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ContentVersion"
	///      },
	///      "minItems": 1
	///    },
	///    "fNums": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "medCompN": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Flows {
		#[serde(rename = "contVers", default, skip_serializing_if = "Vec::is_empty")]
		pub cont_vers: Vec<ContentVersion>,
		#[serde(rename = "fNums", default, skip_serializing_if = "Vec::is_empty")]
		pub f_nums: Vec<i64>,
		#[serde(rename = "medCompN")]
		pub med_comp_n: i64,
	}

	impl From<&Flows> for Flows {
		fn from(value: &Flows) -> Self {
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
	///      "$ref": "#/components/schemas/schemas-TransportProtocol"
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
		pub transport: Option<SchemasTransportProtocol>,
	}

	impl From<&IpEndPoint> for IpEndPoint {
		fn from(value: &IpEndPoint) -> Self {
			value.clone()
		}
	}

	/// Identifies an UL/DL IP flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies an UL/DL IP flow.",
	///  "type": "object",
	///  "required": [
	///    "flowNumber"
	///  ],
	///  "properties": {
	///    "flowNumber": {
	///      "type": "integer"
	///    },
	///    "ipFlows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-FlowDescription"
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
	pub struct IpFlowInfo {
		#[serde(rename = "flowNumber")]
		pub flow_number: i64,
		#[serde(rename = "ipFlows", default, skip_serializing_if = "Vec::is_empty")]
		pub ip_flows: Vec<SchemasFlowDescription>,
	}

	impl From<&IpFlowInfo> for IpFlowInfo {
		fn from(value: &IpFlowInfo) -> Self {
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

	/// Contains the IP multicast addressing information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the IP multicast addressing information.",
	///  "type": "object",
	///  "properties": {
	///    "ipv4MulAddr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6MulAddr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "srcIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "srcIpv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IpMulticastAddressInfo {
		#[serde(
			rename = "ipv4MulAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv4_mul_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "ipv6MulAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_mul_addr: Option<Ipv6Addr>,
		#[serde(
			rename = "srcIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub src_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "srcIpv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub src_ipv6_addr: Option<Ipv6Addr>,
	}

	impl From<&IpMulticastAddressInfo> for IpMulticastAddressInfo {
		fn from(value: &IpMulticastAddressInfo) -> Self {
			value.clone()
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

	/// Contains the mapping of S-NSSAI in the serving network and the value of
	/// the home network
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the mapping of S-NSSAI in the serving network
	/// and the value of the home network",
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
	pub struct MappingOfSnssai {
		#[serde(rename = "homeSnssai")]
		pub home_snssai: Snssai,
		#[serde(rename = "servingSnssai")]
		pub serving_snssai: Snssai,
	}

	impl From<&MappingOfSnssai> for MappingOfSnssai {
		fn from(value: &MappingOfSnssai) -> Self {
			value.clone()
		}
	}

	/// Represents the parameter of an MBS Application Session Context.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the parameter of an MBS Application Session
	/// Context.\n",
	///  "type": "object",
	///  "required": [
	///    "mbsSessionId"
	///  ],
	///  "properties": {
	///    "areaSessPolId": {
	///      "$ref": "#/components/schemas/Uint16"
	///    },
	///    "contactPcfInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mbsServInfo": {
	///      "$ref": "#/components/schemas/MbsServiceInfo"
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    },
	///    "reqForLocDepMbs": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsAppSessionCtxt {
		#[serde(
			rename = "areaSessPolId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub area_sess_pol_id: Option<Uint16>,
		#[serde(rename = "contactPcfInd", default)]
		pub contact_pcf_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mbsServInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_serv_info: Option<MbsServiceInfo>,
		#[serde(rename = "mbsSessionId")]
		pub mbs_session_id: MbsSessionId,
		#[serde(rename = "reqForLocDepMbs", default)]
		pub req_for_loc_dep_mbs: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
	}

	impl From<&MbsAppSessionCtxt> for MbsAppSessionCtxt {
		fn from(value: &MbsAppSessionCtxt) -> Self {
			value.clone()
		}
	}

	/// Represents the modifications to an existing MBS Application Session
	/// Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the modifications to an existing MBS
	/// Application Session Context resource.\n",
	///  "type": "object",
	///  "properties": {
	///    "mbsServInfo": {
	///      "$ref": "#/components/schemas/MbsServiceInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsAppSessionCtxtPatch {
		#[serde(
			rename = "mbsServInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_serv_info: Option<MbsServiceInfo>,
	}

	impl From<&MbsAppSessionCtxtPatch> for MbsAppSessionCtxtPatch {
		fn from(value: &MbsAppSessionCtxtPatch) -> Self {
			value.clone()
		}
	}

	/// Represents the reporting of MBS Policy decision level failure(s) and/or
	/// MBS PCC rule level failure(s).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the reporting of MBS Policy decision level
	/// failure(s) and/or MBS PCC rule level failure(s).\n",
	///  "type": "object",
	///  "properties": {
	///    "mbsReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsReport"
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
	pub struct MbsErrorReport {
		#[serde(rename = "mbsReports", default, skip_serializing_if = "Vec::is_empty")]
		pub mbs_reports: Vec<MbsReport>,
	}

	impl From<&MbsErrorReport> for MbsErrorReport {
		fn from(value: &MbsErrorReport) -> Self {
			value.clone()
		}
	}

	/// Identifies the MBS related extensions to the ProblemDetails data
	/// structure.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the MBS related extensions to the
	/// ProblemDetails data structure.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AcceptableMbsServInfo"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum MbsExtProblemDetails {
		#[default]
		Variant0 {
			#[serde(rename = "accMbsServInfo")]
			acc_mbs_serv_info: ::std::collections::HashMap<String, MbsMediaComp>,
			#[serde(
				rename = "accessTokenError",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			access_token_error: Option<AccessTokenErr>,
			#[serde(
				rename = "accessTokenRequest",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			access_token_request: Option<AccessTokenReq>,
			/// A machine-readable application error cause specific to this
			/// occurrence of the problem.  This IE should be present and
			/// provide application-related error information, if available.
			#[serde(default, skip_serializing_if = "Option::is_none")]
			cause: Option<String>,
			/// A human-readable explanation specific to this occurrence of the
			/// problem.
			#[serde(default, skip_serializing_if = "Option::is_none")]
			detail: Option<String>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			instance: Option<Uri>,
			#[serde(
				rename = "invalidParams",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			invalid_params: Vec<InvalidParam>,
			#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
			nrf_id: Option<Fqdn>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			status: Option<i64>,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			title: Option<String>,
			#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
			type_: Option<Uri>,
		},
		Variant1 {
			#[serde(rename = "accMaxMbsBw")]
			acc_max_mbs_bw: BitRate,
			#[serde(
				rename = "accessTokenError",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			access_token_error: Option<AccessTokenErr>,
			#[serde(
				rename = "accessTokenRequest",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			access_token_request: Option<AccessTokenReq>,
			/// A machine-readable application error cause specific to this
			/// occurrence of the problem.  This IE should be present and
			/// provide application-related error information, if available.
			#[serde(default, skip_serializing_if = "Option::is_none")]
			cause: Option<String>,
			/// A human-readable explanation specific to this occurrence of the
			/// problem.
			#[serde(default, skip_serializing_if = "Option::is_none")]
			detail: Option<String>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			instance: Option<Uri>,
			#[serde(
				rename = "invalidParams",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			invalid_params: Vec<InvalidParam>,
			#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
			nrf_id: Option<Fqdn>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			status: Option<i64>,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			title: Option<String>,
			#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
			type_: Option<Uri>,
		},
	}

	impl From<&MbsExtProblemDetails> for MbsExtProblemDetails {
		fn from(value: &MbsExtProblemDetails) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - NF_MALFUNCTION: Indicates that the MBS PCC rule could not be
	///   successfully installed due to MB-SMF/MB-UPF malfunction.
	/// - NF_RESOURCES_UNAVAILABLE: Indicates that the MBS PCC rule could not be
	///   successfully installed due to resources unavailable at the
	///   MB-SMF/MB-UPF.
	/// - RESOURCE_ALLOCATION_FAILURE: Indicates that the MBS PCC rule could not
	///   be successfully installed or maintained since the associated MBS QoS
	///   flow establishment/modification failed or the associated MBS QoS flow
	///   was released.
	/// - MBS_QOS_VALIDATION_FAILURE: Indicates that MBS QoS validation has
	///   failed.
	/// - NO_MBS_QOS_FLOW: Indicates that there is no MBS QoS flow to which the
	///   MB-SMF can bind the MBS PCC rule(s).
	/// - MBS_QOS_DECISION_ERROR: Indicates failure in the provisioning of MBS
	///   QoS Decision data.
	/// - MBS_POLICY_PARAM_ERROR: Indicates that the information related to the
	///   provisioned MBS policy parameter(s) is incorrect, incomplete or
	///   inconsistent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- NF_MALFUNCTION: Indicates that
	/// the MBS PCC rule could not be successfully installed due to
	/// MB-SMF/MB-UPF malfunction.\n- NF_RESOURCES_UNAVAILABLE: Indicates that
	/// the MBS PCC rule could not be successfully installed due to resources
	/// unavailable at the MB-SMF/MB-UPF.\n- RESOURCE_ALLOCATION_FAILURE:
	/// Indicates that the MBS PCC rule could not be successfully installed or
	/// maintained since the associated MBS QoS flow establishment/modification
	/// failed or the associated MBS QoS flow was released.\n-
	/// MBS_QOS_VALIDATION_FAILURE: Indicates that MBS QoS validation has
	/// failed.\n- NO_MBS_QOS_FLOW: Indicates that there is no MBS QoS flow to
	/// which the MB-SMF can bind the MBS PCC rule(s).\n-
	/// MBS_QOS_DECISION_ERROR: Indicates failure in the provisioning of MBS QoS
	/// Decision data.\n- MBS_POLICY_PARAM_ERROR: Indicates that the information
	/// related to the provisioned MBS policy parameter(s) is incorrect,
	/// incomplete or inconsistent.\n",
	///  "type": "string",
	///  "enum": [
	///    "NF_MALFUNCTION",
	///    "NF_RESOURCES_UNAVAILABLE",
	///    "RESOURCE_ALLOCATION_FAILURE",
	///    "MBS_QOS_VALIDATION_FAILURE",
	///    "NO_MBS_QOS_FLOW",
	///    "MBS_QOS_DECISION_ERROR",
	///    "MBS_POLICY_PARAM_ERROR"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum MbsFailureCode {
		#[default]
		#[serde(rename = "NF_MALFUNCTION")]
		NfMalfunction,
		#[serde(rename = "NF_RESOURCES_UNAVAILABLE")]
		NfResourcesUnavailable,
		#[serde(rename = "RESOURCE_ALLOCATION_FAILURE")]
		ResourceAllocationFailure,
		#[serde(rename = "MBS_QOS_VALIDATION_FAILURE")]
		MbsQosValidationFailure,
		#[serde(rename = "NO_MBS_QOS_FLOW")]
		NoMbsQosFlow,
		#[serde(rename = "MBS_QOS_DECISION_ERROR")]
		MbsQosDecisionError,
		#[serde(rename = "MBS_POLICY_PARAM_ERROR")]
		MbsPolicyParamError,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsFailureCode> for MbsFailureCode {
		fn from(value: &MbsFailureCode) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsFailureCode {
		fn to_string(&self) -> String {
			match *self {
				Self::NfMalfunction => "NF_MALFUNCTION".to_string(),
				Self::NfResourcesUnavailable => "NF_RESOURCES_UNAVAILABLE".to_string(),
				Self::ResourceAllocationFailure => "RESOURCE_ALLOCATION_FAILURE".to_string(),
				Self::MbsQosValidationFailure => "MBS_QOS_VALIDATION_FAILURE".to_string(),
				Self::NoMbsQosFlow => "NO_MBS_QOS_FLOW".to_string(),
				Self::MbsQosDecisionError => "MBS_QOS_DECISION_ERROR".to_string(),
				Self::MbsPolicyParamError => "MBS_POLICY_PARAM_ERROR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsFailureCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_MALFUNCTION" => Ok(Self::NfMalfunction),
				"NF_RESOURCES_UNAVAILABLE" => Ok(Self::NfResourcesUnavailable),
				"RESOURCE_ALLOCATION_FAILURE" => Ok(Self::ResourceAllocationFailure),
				"MBS_QOS_VALIDATION_FAILURE" => Ok(Self::MbsQosValidationFailure),
				"NO_MBS_QOS_FLOW" => Ok(Self::NoMbsQosFlow),
				"MBS_QOS_DECISION_ERROR" => Ok(Self::MbsQosDecisionError),
				"MBS_POLICY_PARAM_ERROR" => Ok(Self::MbsPolicyParamError),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the MBS Maximum Data Burst Volume expressed in Bytes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the MBS Maximum Data Burst Volume expressed
	/// in Bytes.",
	///  "type": "integer",
	///  "maximum": 2000000.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsMaxDataBurstVol(pub i64);

	impl ::std::ops::Deref for MbsMaxDataBurstVol {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<MbsMaxDataBurstVol> for i64 {
		fn from(value: MbsMaxDataBurstVol) -> Self {
			value.0
		}
	}

	impl From<&MbsMaxDataBurstVol> for MbsMaxDataBurstVol {
		fn from(value: &MbsMaxDataBurstVol) -> Self {
			value.clone()
		}
	}

	impl From<i64> for MbsMaxDataBurstVol {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MbsMaxDataBurstVol {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MbsMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MbsMaxDataBurstVol {
		fn to_string(&self) -> String {
			self.0.to_string()
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
	///        "$ref": "#/components/schemas/schemas-FlowDescription"
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
		pub mbs_flow_descs: Vec<SchemasFlowDescription>,
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

	/// Represents the parameters constituting an MBS PCC rule.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the parameters constituting an MBS PCC
	/// rule.",
	///  "type": "object",
	///  "required": [
	///    "mbsPccRuleId"
	///  ],
	///  "properties": {
	///    "mbsDlIpFlowInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsPccRuleId": {
	///      "type": "string"
	///    },
	///    "precedence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "refMbsQosDec": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsPccRule {
		#[serde(
			rename = "mbsDlIpFlowInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbs_dl_ip_flow_info: Vec<FlowDescription>,
		#[serde(rename = "mbsPccRuleId")]
		pub mbs_pcc_rule_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub precedence: Option<Uinteger>,
		#[serde(
			rename = "refMbsQosDec",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_mbs_qos_dec: Option<[String; 1usize]>,
	}

	impl From<&MbsPccRule> for MbsPccRule {
		fn from(value: &MbsPccRule) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - ACTIVE: Indicates that the MBS PCC rule(s) are successfully installed.
	/// - INACTIVE: Indicates that the MBS PCC rule(s) are removed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- ACTIVE: Indicates that the MBS
	/// PCC rule(s) are successfully installed.\n- INACTIVE: Indicates that the
	/// MBS PCC rule(s) are removed.\n",
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
	pub enum MbsPccRuleStatus {
		#[default]
		#[serde(rename = "ACTIVE")]
		Active,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsPccRuleStatus> for MbsPccRuleStatus {
		fn from(value: &MbsPccRuleStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsPccRuleStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Active => "ACTIVE".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsPccRuleStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE" => Ok(Self::Active),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsPccRuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsPccRuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsPccRuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are
	/// - MBS_SESSION_UPDATE: Indicates the MBS Session Update policy control
	///   request trigger.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- MBS_SESSION_UPDATE: Indicates
	/// the MBS Session Update policy control request trigger.\n",
	///  "type": "string",
	///  "enum": [
	///    "MBS_SESSION_UPDATE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum MbsPcrt {
		#[default]
		#[serde(rename = "MBS_SESSION_UPDATE")]
		MbsSessionUpdate,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsPcrt> for MbsPcrt {
		fn from(value: &MbsPcrt) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsPcrt {
		fn to_string(&self) -> String {
			match *self {
				Self::MbsSessionUpdate => "MBS_SESSION_UPDATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsPcrt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MBS_SESSION_UPDATE" => Ok(Self::MbsSessionUpdate),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsPcrt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsPcrt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsPcrt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the parameters used to request the creation of an MBS Policy
	/// Association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the parameters used to request the creation of
	/// an MBS Policy Association.\n",
	///  "type": "object",
	///  "required": [
	///    "mbsSessionId"
	///  ],
	///  "properties": {
	///    "areaSessPolId": {
	///      "$ref": "#/components/schemas/Uint16"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mbsServInfo": {
	///      "$ref": "#/components/schemas/MbsServiceInfo"
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsPolicyCtxtData {
		#[serde(
			rename = "areaSessPolId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub area_sess_pol_id: Option<Uint16>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mbsServInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_serv_info: Option<MbsServiceInfo>,
		#[serde(rename = "mbsSessionId")]
		pub mbs_session_id: MbsSessionId,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
	}

	impl From<&MbsPolicyCtxtData> for MbsPolicyCtxtData {
		fn from(value: &MbsPolicyCtxtData) -> Self {
			value.clone()
		}
	}

	/// Contains the parameters to request the modification of an existing MBS
	/// Policy Association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the parameters to request the modification of
	/// an existing MBS Policy Association.\n",
	///  "type": "object",
	///  "properties": {
	///    "mbsErrorReport": {
	///      "$ref": "#/components/schemas/MbsErrorReport"
	///    },
	///    "mbsPcrts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsPcrt"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsServInfo": {
	///      "$ref": "#/components/schemas/MbsServiceInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsPolicyCtxtDataUpdate {
		#[serde(
			rename = "mbsErrorReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_error_report: Option<MbsErrorReport>,
		#[serde(rename = "mbsPcrts", default, skip_serializing_if = "Vec::is_empty")]
		pub mbs_pcrts: Vec<MbsPcrt>,
		#[serde(
			rename = "mbsServInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_serv_info: Option<MbsServiceInfo>,
	}

	impl From<&MbsPolicyCtxtDataUpdate> for MbsPolicyCtxtDataUpdate {
		fn from(value: &MbsPolicyCtxtDataUpdate) -> Self {
			value.clone()
		}
	}

	/// Contains the MBS policy data provisioned as part of an MBS Policy
	/// Association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the MBS policy data provisioned as part of an
	/// MBS Policy Association.\n",
	///  "type": "object",
	///  "required": [
	///    "mbsPolicyCtxtData"
	///  ],
	///  "properties": {
	///    "mbsPolicies": {
	///      "$ref": "#/components/schemas/MbsPolicyDecision"
	///    },
	///    "mbsPolicyCtxtData": {
	///      "$ref": "#/components/schemas/MbsPolicyCtxtData"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsPolicyData {
		#[serde(
			rename = "mbsPolicies",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_policies: Option<MbsPolicyDecision>,
		#[serde(rename = "mbsPolicyCtxtData")]
		pub mbs_policy_ctxt_data: MbsPolicyCtxtData,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
	}

	impl From<&MbsPolicyData> for MbsPolicyData {
		fn from(value: &MbsPolicyData) -> Self {
			value.clone()
		}
	}

	/// Represents the parameters constituting an MBS Policy Decision.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the parameters constituting an MBS Policy
	/// Decision.\n",
	///  "type": "object",
	///  "properties": {
	///    "authMbsSessAmbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mbsPccRules": {
	///      "description": "A map of MBS PCC rule(s) with each map entry
	/// containing the MbsPccRule data structure. The key of the map for each
	/// entry is the mbsPccRuleId attribute of the corresponding MbsPccRule data
	/// structure.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsPccRule"
	///      }
	///    },
	///    "mbsPcrts": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/MbsPcrt"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsQosChars": {
	///      "description": "A map of MBS QoS Characteristics set(s) with each
	/// map entry containing the MbsQosChar data structure. The key of the map
	/// for each entry is the 5QI attribute of the corresponding MbsQosDec data
	/// structure.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsQosChar"
	///      }
	///    },
	///    "mbsQosDecs": {
	///      "description": "A map of MBS QoS Decision(s) with each map entry
	/// containing the MbsQosDec data structure. The key of the map for each
	/// entry is the mbsQosId attribute of the corresponding MbsQosDec data
	/// structure.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsQosDec"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsPolicyDecision {
		#[serde(
			rename = "authMbsSessAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_mbs_sess_ambr: Option<BitRate>,
		/// A map of MBS PCC rule(s) with each map entry containing the
		/// MbsPccRule data structure. The key of the map for each entry is the
		/// mbsPccRuleId attribute of the corresponding MbsPccRule data
		/// structure.
		#[serde(
			rename = "mbsPccRules",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_pcc_rules: Option<::std::collections::HashMap<String, MbsPccRule>>,
		#[serde(rename = "mbsPcrts", default, skip_serializing_if = "Option::is_none")]
		pub mbs_pcrts: Option<Vec<MbsPcrt>>,
		/// A map of MBS QoS Characteristics set(s) with each map entry
		/// containing the MbsQosChar data structure. The key of the map for
		/// each entry is the 5QI attribute of the corresponding MbsQosDec data
		/// structure.
		#[serde(
			rename = "mbsQosChars",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_qos_chars: ::std::collections::HashMap<String, MbsQosChar>,
		/// A map of MBS QoS Decision(s) with each map entry containing the
		/// MbsQosDec data structure. The key of the map for each entry is the
		/// mbsQosId attribute of the corresponding MbsQosDec data structure.
		#[serde(
			rename = "mbsQosDecs",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_qos_decs: ::std::collections::HashMap<String, MbsQosDec>,
	}

	impl From<&MbsPolicyDecision> for MbsPolicyDecision {
		fn from(value: &MbsPolicyDecision) -> Self {
			value.clone()
		}
	}

	/// Represents the parameters constituting a set of explicitly signalled QoS
	/// characteristics.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the parameters constituting a set of
	/// explicitly signalled QoS characteristics.\n",
	///  "type": "object",
	///  "required": [
	///    "5qi",
	///    "mbsMaxDataBurstVol",
	///    "packetDelayBudget",
	///    "packetErrorRate",
	///    "priorityLevel",
	///    "resourceType"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "mbsMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/MbsMaxDataBurstVol"
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
	pub struct MbsQosChar {
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindow>,
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(rename = "mbsMaxDataBurstVol")]
		pub mbs_max_data_burst_vol: MbsMaxDataBurstVol,
		#[serde(rename = "packetDelayBudget")]
		pub packet_delay_budget: PacketDelBudget,
		#[serde(rename = "packetErrorRate")]
		pub packet_error_rate: PacketErrRate,
		#[serde(rename = "priorityLevel")]
		pub priority_level: _5qiPriorityLevel,
		#[serde(rename = "resourceType")]
		pub resource_type: QosResourceType,
	}

	impl From<&MbsQosChar> for MbsQosChar {
		fn from(value: &MbsQosChar) -> Self {
			value.clone()
		}
	}

	/// Represents the parameters constituting an MBS QoS Decision.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the parameters constituting an MBS QoS
	/// Decision.",
	///  "type": "object",
	///  "required": [
	///    "mbsQosId"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mbsMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/MbsMaxDataBurstVol"
	///    },
	///    "mbsQosId": {
	///      "type": "string"
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
	pub struct MbsQosDec {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindow>,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(rename = "gbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_dl: Option<BitRate>,
		#[serde(rename = "mbrDl", default, skip_serializing_if = "Option::is_none")]
		pub mbr_dl: Option<BitRate>,
		#[serde(
			rename = "mbsMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_max_data_burst_vol: Option<MbsMaxDataBurstVol>,
		#[serde(rename = "mbsQosId")]
		pub mbs_qos_id: String,
		#[serde(
			rename = "priorityLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub priority_level: Option<_5qiPriorityLevel>,
	}

	impl From<&MbsQosDec> for MbsQosDec {
		fn from(value: &MbsQosDec) -> Self {
			value.clone()
		}
	}

	/// Contains information about the MBS Policy Decision level failure(s)
	/// and/or the MBS PCC rule level failure(s).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information about the MBS Policy Decision
	/// level failure(s) and/or the MBS PCC rule level failure(s).\n",
	///  "type": "object",
	///  "properties": {
	///    "failureCode": {
	///      "$ref": "#/components/schemas/MbsFailureCode"
	///    },
	///    "mbsPccRuleIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsPccRuleStatus": {
	///      "$ref": "#/components/schemas/MbsPccRuleStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsReport {
		#[serde(
			rename = "failureCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failure_code: Option<MbsFailureCode>,
		#[serde(
			rename = "mbsPccRuleIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbs_pcc_rule_ids: Vec<String>,
		#[serde(
			rename = "mbsPccRuleStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_pcc_rule_status: Option<MbsPccRuleStatus>,
	}

	impl From<&MbsReport> for MbsReport {
		fn from(value: &MbsReport) -> Self {
			value.clone()
		}
	}

	/// Identifies a media component.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies a media component.",
	///  "type": "object",
	///  "required": [
	///    "medCompN"
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "afRoutReq": {
	///      "$ref": "#/components/schemas/AfRoutingRequirement"
	///    },
	///    "altSerReqs": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "altSerReqsData": {
	///      "description": "Contains alternative service requirements that
	/// include individual QoS parameter sets.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AlternativeServiceRequirementsData"
	///      },
	///      "minItems": 1
	///    },
	///    "codecs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodecData"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "desMaxLatency": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "desMaxLoss": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "disUeNotif": {
	///      "type": "boolean"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flusId": {
	///      "type": "string"
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxSuppBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxSuppBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "medCompN": {
	///      "type": "integer"
	///    },
	///    "medSubComps": {
	///      "description": "Contains the requested bitrate and filters for the
	/// set of service data flows identified by their common flow identifier.
	/// The key of the map is the fNum attribute.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaSubComponent"
	///      }
	///    },
	///    "medType": {
	///      "$ref": "#/components/schemas/MediaType"
	///    },
	///    "minDesBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "minDesBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mirBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mirBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "preemptCap": {
	///      "$ref": "#/components/schemas/PreemptionCapability"
	///    },
	///    "preemptVuln": {
	///      "$ref": "#/components/schemas/PreemptionVulnerability"
	///    },
	///    "prioSharingInd": {
	///      "$ref": "#/components/schemas/PrioritySharingIndicator"
	///    },
	///    "qosReference": {
	///      "type": "string"
	///    },
	///    "resPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "rrBw": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "rsBw": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "sharingKeyDl": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sharingKeyUl": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "tscaiInputDl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiInputUl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiTimeDom": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "tsnQos": {
	///      "$ref": "#/components/schemas/TsnQosContainer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaComponent {
		#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
		pub af_app_id: Option<AfAppId>,
		#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
		pub af_rout_req: Option<AfRoutingRequirement>,
		#[serde(rename = "altSerReqs", default, skip_serializing_if = "Vec::is_empty")]
		pub alt_ser_reqs: Vec<String>,
		/// Contains alternative service requirements that include individual
		/// QoS parameter sets.
		#[serde(
			rename = "altSerReqsData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_ser_reqs_data: Vec<AlternativeServiceRequirementsData>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub codecs: Vec<CodecData>,
		#[serde(rename = "contVer", default, skip_serializing_if = "Option::is_none")]
		pub cont_ver: Option<ContentVersion>,
		#[serde(
			rename = "desMaxLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub des_max_latency: Option<Float>,
		#[serde(
			rename = "desMaxLoss",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub des_max_loss: Option<Float>,
		#[serde(
			rename = "disUeNotif",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dis_ue_notif: Option<bool>,
		#[serde(rename = "fStatus", default, skip_serializing_if = "Option::is_none")]
		pub f_status: Option<FlowStatus>,
		#[serde(rename = "flusId", default, skip_serializing_if = "Option::is_none")]
		pub flus_id: Option<String>,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRate>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRate>,
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
		#[serde(
			rename = "maxSuppBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_supp_bw_dl: Option<BitRate>,
		#[serde(
			rename = "maxSuppBwUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_supp_bw_ul: Option<BitRate>,
		#[serde(rename = "medCompN")]
		pub med_comp_n: i64,
		/// Contains the requested bitrate and filters for the set of service
		/// data flows identified by their common flow identifier. The key of
		/// the map is the fNum attribute.
		#[serde(
			rename = "medSubComps",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub med_sub_comps: ::std::collections::HashMap<String, MediaSubComponent>,
		#[serde(rename = "medType", default, skip_serializing_if = "Option::is_none")]
		pub med_type: Option<MediaType>,
		#[serde(
			rename = "minDesBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_des_bw_dl: Option<BitRate>,
		#[serde(
			rename = "minDesBwUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_des_bw_ul: Option<BitRate>,
		#[serde(rename = "mirBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_dl: Option<BitRate>,
		#[serde(rename = "mirBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_ul: Option<BitRate>,
		#[serde(
			rename = "preemptCap",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preempt_cap: Option<PreemptionCapability>,
		#[serde(
			rename = "preemptVuln",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preempt_vuln: Option<PreemptionVulnerability>,
		#[serde(
			rename = "prioSharingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prio_sharing_ind: Option<PrioritySharingIndicator>,
		#[serde(
			rename = "qosReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_reference: Option<String>,
		#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
		pub res_prio: Option<ReservPriority>,
		#[serde(rename = "rrBw", default, skip_serializing_if = "Option::is_none")]
		pub rr_bw: Option<BitRate>,
		#[serde(rename = "rsBw", default, skip_serializing_if = "Option::is_none")]
		pub rs_bw: Option<BitRate>,
		#[serde(
			rename = "sharingKeyDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_dl: Option<Uint32>,
		#[serde(
			rename = "sharingKeyUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_ul: Option<Uint32>,
		#[serde(
			rename = "tscaiInputDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_dl: Option<TscaiInputContainer>,
		#[serde(
			rename = "tscaiInputUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_ul: Option<TscaiInputContainer>,
		#[serde(
			rename = "tscaiTimeDom",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_time_dom: Option<Uinteger>,
		#[serde(rename = "tsnQos", default, skip_serializing_if = "Option::is_none")]
		pub tsn_qos: Option<TsnQosContainer>,
	}

	impl From<&MediaComponent> for MediaComponent {
		fn from(value: &MediaComponent) -> Self {
			value.clone()
		}
	}

	/// Indicates whether the media component is active or inactive.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the media component is active or
	/// inactive.",
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
	pub enum MediaComponentResourcesStatus {
		#[default]
		#[serde(rename = "ACTIVE")]
		Active,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MediaComponentResourcesStatus> for MediaComponentResourcesStatus {
		fn from(value: &MediaComponentResourcesStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for MediaComponentResourcesStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Active => "ACTIVE".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MediaComponentResourcesStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE" => Ok(Self::Active),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MediaComponentResourcesStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MediaComponentResourcesStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MediaComponentResourcesStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// This data type is defined in the same way as the MediaComponent data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// MediaComponent data type, but with the OpenAPI nullable property set to
	/// true.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "medCompN"
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "afRoutReq": {
	///      "$ref": "#/components/schemas/AfRoutingRequirementRm"
	///    },
	///    "altSerReqs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "altSerReqsData": {
	///      "description": "Contains removable alternative service requirements
	/// that include individual QoS parameter sets.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/AlternativeServiceRequirementsData"
	///      },
	///      "minItems": 1
	///    },
	///    "codecs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodecData"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "desMaxLatency": {
	///      "$ref": "#/components/schemas/FloatRm"
	///    },
	///    "desMaxLoss": {
	///      "$ref": "#/components/schemas/FloatRm"
	///    },
	///    "disUeNotif": {
	///      "type": "boolean"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flusId": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxSuppBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxSuppBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "medCompN": {
	///      "type": "integer"
	///    },
	///    "medSubComps": {
	///      "description": "Contains the requested bitrate and filters for the
	/// set of service data flows identified by their common flow identifier.
	/// The key of the map is the fNum attribute.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaSubComponentRm"
	///      }
	///    },
	///    "medType": {
	///      "$ref": "#/components/schemas/MediaType"
	///    },
	///    "minDesBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "minDesBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "mirBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "mirBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "preemptCap": {
	///      "$ref": "#/components/schemas/PreemptionCapabilityRm"
	///    },
	///    "preemptVuln": {
	///      "$ref": "#/components/schemas/PreemptionVulnerabilityRm"
	///    },
	///    "prioSharingInd": {
	///      "$ref": "#/components/schemas/PrioritySharingIndicator"
	///    },
	///    "qosReference": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "resPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "rrBw": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "rsBw": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "sharingKeyDl": {
	///      "$ref": "#/components/schemas/Uint32Rm"
	///    },
	///    "sharingKeyUl": {
	///      "$ref": "#/components/schemas/Uint32Rm"
	///    },
	///    "tscaiInputDl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiInputUl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiTimeDom": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "tsnQos": {
	///      "$ref": "#/components/schemas/TsnQosContainerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaComponentRm(pub Option<MediaComponentRmInner>);

	impl ::std::ops::Deref for MediaComponentRm {
		type Target = Option<MediaComponentRmInner>;
		fn deref(&self) -> &Option<MediaComponentRmInner> {
			&self.0
		}
	}

	impl From<MediaComponentRm> for Option<MediaComponentRmInner> {
		fn from(value: MediaComponentRm) -> Self {
			value.0
		}
	}

	impl From<&MediaComponentRm> for MediaComponentRm {
		fn from(value: &MediaComponentRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<MediaComponentRmInner>> for MediaComponentRm {
		fn from(value: Option<MediaComponentRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the MediaComponent data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// MediaComponent data type, but with the OpenAPI nullable property set to
	/// true.",
	///  "type": "object",
	///  "required": [
	///    "medCompN"
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "afRoutReq": {
	///      "$ref": "#/components/schemas/AfRoutingRequirementRm"
	///    },
	///    "altSerReqs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "altSerReqsData": {
	///      "description": "Contains removable alternative service requirements
	/// that include individual QoS parameter sets.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/AlternativeServiceRequirementsData"
	///      },
	///      "minItems": 1
	///    },
	///    "codecs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodecData"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "desMaxLatency": {
	///      "$ref": "#/components/schemas/FloatRm"
	///    },
	///    "desMaxLoss": {
	///      "$ref": "#/components/schemas/FloatRm"
	///    },
	///    "disUeNotif": {
	///      "type": "boolean"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flusId": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxSuppBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxSuppBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "medCompN": {
	///      "type": "integer"
	///    },
	///    "medSubComps": {
	///      "description": "Contains the requested bitrate and filters for the
	/// set of service data flows identified by their common flow identifier.
	/// The key of the map is the fNum attribute.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MediaSubComponentRm"
	///      }
	///    },
	///    "medType": {
	///      "$ref": "#/components/schemas/MediaType"
	///    },
	///    "minDesBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "minDesBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "mirBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "mirBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "preemptCap": {
	///      "$ref": "#/components/schemas/PreemptionCapabilityRm"
	///    },
	///    "preemptVuln": {
	///      "$ref": "#/components/schemas/PreemptionVulnerabilityRm"
	///    },
	///    "prioSharingInd": {
	///      "$ref": "#/components/schemas/PrioritySharingIndicator"
	///    },
	///    "qosReference": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "resPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "rrBw": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "rsBw": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "sharingKeyDl": {
	///      "$ref": "#/components/schemas/Uint32Rm"
	///    },
	///    "sharingKeyUl": {
	///      "$ref": "#/components/schemas/Uint32Rm"
	///    },
	///    "tscaiInputDl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiInputUl": {
	///      "$ref": "#/components/schemas/TscaiInputContainer"
	///    },
	///    "tscaiTimeDom": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "tsnQos": {
	///      "$ref": "#/components/schemas/TsnQosContainerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaComponentRmInner {
		#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
		pub af_app_id: Option<AfAppId>,
		#[serde(rename = "afRoutReq", default, skip_serializing_if = "Option::is_none")]
		pub af_rout_req: Option<AfRoutingRequirementRm>,
		#[serde(
			rename = "altSerReqs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_ser_reqs: Option<Vec<String>>,
		/// Contains removable alternative service requirements that include
		/// individual QoS parameter sets.
		#[serde(
			rename = "altSerReqsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_ser_reqs_data: Option<Vec<AlternativeServiceRequirementsData>>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub codecs: Vec<CodecData>,
		#[serde(rename = "contVer", default, skip_serializing_if = "Option::is_none")]
		pub cont_ver: Option<ContentVersion>,
		#[serde(
			rename = "desMaxLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub des_max_latency: Option<FloatRm>,
		#[serde(
			rename = "desMaxLoss",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub des_max_loss: Option<FloatRm>,
		#[serde(
			rename = "disUeNotif",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dis_ue_notif: Option<bool>,
		#[serde(rename = "fStatus", default, skip_serializing_if = "Option::is_none")]
		pub f_status: Option<FlowStatus>,
		#[serde(rename = "flusId", default, skip_serializing_if = "Option::is_none")]
		pub flus_id: Option<String>,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRateRm>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRateRm>,
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
		#[serde(
			rename = "maxSuppBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_supp_bw_dl: Option<BitRateRm>,
		#[serde(
			rename = "maxSuppBwUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_supp_bw_ul: Option<BitRateRm>,
		#[serde(rename = "medCompN")]
		pub med_comp_n: i64,
		/// Contains the requested bitrate and filters for the set of service
		/// data flows identified by their common flow identifier. The key of
		/// the map is the fNum attribute.
		#[serde(
			rename = "medSubComps",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub med_sub_comps: ::std::collections::HashMap<String, MediaSubComponentRm>,
		#[serde(rename = "medType", default, skip_serializing_if = "Option::is_none")]
		pub med_type: Option<MediaType>,
		#[serde(
			rename = "minDesBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_des_bw_dl: Option<BitRateRm>,
		#[serde(
			rename = "minDesBwUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_des_bw_ul: Option<BitRateRm>,
		#[serde(rename = "mirBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_dl: Option<BitRateRm>,
		#[serde(rename = "mirBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_ul: Option<BitRateRm>,
		#[serde(
			rename = "preemptCap",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preempt_cap: Option<PreemptionCapabilityRm>,
		#[serde(
			rename = "preemptVuln",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preempt_vuln: Option<PreemptionVulnerabilityRm>,
		#[serde(
			rename = "prioSharingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prio_sharing_ind: Option<PrioritySharingIndicator>,
		#[serde(
			rename = "qosReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_reference: Option<String>,
		#[serde(rename = "resPrio", default, skip_serializing_if = "Option::is_none")]
		pub res_prio: Option<ReservPriority>,
		#[serde(rename = "rrBw", default, skip_serializing_if = "Option::is_none")]
		pub rr_bw: Option<BitRateRm>,
		#[serde(rename = "rsBw", default, skip_serializing_if = "Option::is_none")]
		pub rs_bw: Option<BitRateRm>,
		#[serde(
			rename = "sharingKeyDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_dl: Option<Uint32Rm>,
		#[serde(
			rename = "sharingKeyUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_ul: Option<Uint32Rm>,
		#[serde(
			rename = "tscaiInputDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_dl: Option<TscaiInputContainer>,
		#[serde(
			rename = "tscaiInputUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_ul: Option<TscaiInputContainer>,
		#[serde(
			rename = "tscaiTimeDom",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_time_dom: Option<Uinteger>,
		#[serde(rename = "tsnQos", default, skip_serializing_if = "Option::is_none")]
		pub tsn_qos: Option<TsnQosContainerRm>,
	}

	impl From<&MediaComponentRmInner> for MediaComponentRmInner {
		fn from(value: &MediaComponentRmInner) -> Self {
			value.clone()
		}
	}

	/// Identifies a media subcomponent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies a media subcomponent.",
	///  "type": "object",
	///  "required": [
	///    "fNum"
	///  ],
	///  "properties": {
	///    "afSigProtocol": {
	///      "$ref": "#/components/schemas/AfSigProtocol"
	///    },
	///    "ethfDescs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fDescs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription1"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fNum": {
	///      "type": "integer"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flowUsage": {
	///      "$ref": "#/components/schemas/FlowUsage"
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "tosTrCl": {
	///      "$ref": "#/components/schemas/TosTrafficClass"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaSubComponent {
		#[serde(
			rename = "afSigProtocol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_sig_protocol: Option<AfSigProtocol>,
		#[serde(rename = "ethfDescs", default, skip_serializing_if = "Vec::is_empty")]
		pub ethf_descs: Vec<EthFlowDescription>,
		#[serde(rename = "fDescs", default, skip_serializing_if = "Vec::is_empty")]
		pub f_descs: Vec<FlowDescription1>,
		#[serde(rename = "fNum")]
		pub f_num: i64,
		#[serde(rename = "fStatus", default, skip_serializing_if = "Option::is_none")]
		pub f_status: Option<FlowStatus>,
		#[serde(rename = "flowUsage", default, skip_serializing_if = "Option::is_none")]
		pub flow_usage: Option<FlowUsage>,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRate>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRate>,
		#[serde(rename = "tosTrCl", default, skip_serializing_if = "Option::is_none")]
		pub tos_tr_cl: Option<TosTrafficClass>,
	}

	impl From<&MediaSubComponent> for MediaSubComponent {
		fn from(value: &MediaSubComponent) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the MediaSubComponent data
	/// type, but with the OpenAPI nullable property set to true. Removable
	/// attributes marBwDl and marBwUl are defined with the corresponding
	/// removable data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// MediaSubComponent data type, but with the OpenAPI nullable property set
	/// to true. Removable attributes marBwDl and marBwUl are defined with the
	/// corresponding removable data type.\n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "fNum"
	///  ],
	///  "properties": {
	///    "afSigProtocol": {
	///      "$ref": "#/components/schemas/AfSigProtocol"
	///    },
	///    "ethfDescs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fDescs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription1"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fNum": {
	///      "type": "integer"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flowUsage": {
	///      "$ref": "#/components/schemas/FlowUsage"
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "tosTrCl": {
	///      "$ref": "#/components/schemas/TosTrafficClassRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaSubComponentRm(pub Option<MediaSubComponentRmInner>);

	impl ::std::ops::Deref for MediaSubComponentRm {
		type Target = Option<MediaSubComponentRmInner>;
		fn deref(&self) -> &Option<MediaSubComponentRmInner> {
			&self.0
		}
	}

	impl From<MediaSubComponentRm> for Option<MediaSubComponentRmInner> {
		fn from(value: MediaSubComponentRm) -> Self {
			value.0
		}
	}

	impl From<&MediaSubComponentRm> for MediaSubComponentRm {
		fn from(value: &MediaSubComponentRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<MediaSubComponentRmInner>> for MediaSubComponentRm {
		fn from(value: Option<MediaSubComponentRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the MediaSubComponent data
	/// type, but with the OpenAPI nullable property set to true. Removable
	/// attributes marBwDl and marBwUl are defined with the corresponding
	/// removable data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// MediaSubComponent data type, but with the OpenAPI nullable property set
	/// to true. Removable attributes marBwDl and marBwUl are defined with the
	/// corresponding removable data type.\n",
	///  "type": "object",
	///  "required": [
	///    "fNum"
	///  ],
	///  "properties": {
	///    "afSigProtocol": {
	///      "$ref": "#/components/schemas/AfSigProtocol"
	///    },
	///    "ethfDescs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fDescs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription1"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "fNum": {
	///      "type": "integer"
	///    },
	///    "fStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "flowUsage": {
	///      "$ref": "#/components/schemas/FlowUsage"
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "tosTrCl": {
	///      "$ref": "#/components/schemas/TosTrafficClassRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MediaSubComponentRmInner {
		#[serde(
			rename = "afSigProtocol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_sig_protocol: Option<AfSigProtocol>,
		#[serde(rename = "ethfDescs", default, skip_serializing_if = "Option::is_none")]
		pub ethf_descs: Option<Vec<EthFlowDescription>>,
		#[serde(rename = "fDescs", default, skip_serializing_if = "Option::is_none")]
		pub f_descs: Option<Vec<FlowDescription1>>,
		#[serde(rename = "fNum")]
		pub f_num: i64,
		#[serde(rename = "fStatus", default, skip_serializing_if = "Option::is_none")]
		pub f_status: Option<FlowStatus>,
		#[serde(rename = "flowUsage", default, skip_serializing_if = "Option::is_none")]
		pub flow_usage: Option<FlowUsage>,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRateRm>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRateRm>,
		#[serde(rename = "tosTrCl", default, skip_serializing_if = "Option::is_none")]
		pub tos_tr_cl: Option<TosTrafficClassRm>,
	}

	impl From<&MediaSubComponentRmInner> for MediaSubComponentRmInner {
		fn from(value: &MediaSubComponentRmInner) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - DURATION: Indicates that the duration of the service data flow traffic
	///   shall be metered.
	/// - VOLUME: Indicates that volume of the service data flow traffic shall
	///   be metered.
	/// - DURATION_VOLUME: Indicates that the duration and the volume of the
	///   service data flow traffic shall be metered.
	/// - EVENT: Indicates that events of the service data flow traffic shall be
	///   metered.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- DURATION: Indicates that the
	/// duration of the service data flow traffic shall be metered.\n- VOLUME:
	/// Indicates that volume of the service data flow traffic shall be
	/// metered.\n- DURATION_VOLUME: Indicates that the duration and the volume
	/// of the service data flow traffic shall be metered.\n- EVENT: Indicates
	/// that events of the service data flow traffic shall be metered.\n",
	///  "anyOf": [
	///    {
	///      "type": "string",
	///      "enum": [
	///        "DURATION",
	///        "VOLUME",
	///        "DURATION_VOLUME",
	///        "EVENT"
	///      ]
	///    },
	///    {
	///      "$ref": "#/components/schemas/NullValue"
	///    },
	///    {
	///      "description": "This string provides forward-compatibility with
	/// future extensions to the enumeration but is not used to encode content
	/// defined in the present version of this API.\n",
	///      "type": "string"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MeteringMethod {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<MeteringMethodSubtype0>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<NullValue>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_2: Option<String>,
	}

	impl From<&MeteringMethod> for MeteringMethod {
		fn from(value: &MeteringMethod) -> Self {
			value.clone()
		}
	}

	/// MeteringMethodSubtype0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DURATION",
	///    "VOLUME",
	///    "DURATION_VOLUME",
	///    "EVENT"
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
	pub enum MeteringMethodSubtype0 {
		#[default]
		#[serde(rename = "DURATION")]
		Duration,
		#[serde(rename = "VOLUME")]
		Volume,
		#[serde(rename = "DURATION_VOLUME")]
		DurationVolume,
		#[serde(rename = "EVENT")]
		Event,
	}

	impl From<&MeteringMethodSubtype0> for MeteringMethodSubtype0 {
		fn from(value: &MeteringMethodSubtype0) -> Self {
			value.clone()
		}
	}

	impl ToString for MeteringMethodSubtype0 {
		fn to_string(&self) -> String {
			match *self {
				Self::Duration => "DURATION".to_string(),
				Self::Volume => "VOLUME".to_string(),
				Self::DurationVolume => "DURATION_VOLUME".to_string(),
				Self::Event => "EVENT".to_string(),
			}
		}
	}

	impl std::str::FromStr for MeteringMethodSubtype0 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DURATION" => Ok(Self::Duration),
				"VOLUME" => Ok(Self::Volume),
				"DURATION_VOLUME" => Ok(Self::DurationVolume),
				"EVENT" => Ok(Self::Event),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MeteringMethodSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MeteringMethodSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MeteringMethodSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates whether it is an invocation, a revocation or an invocation
	/// with authorization of the MPS for DTS service.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether it is an invocation, a revocation or
	/// an invocation with authorization of the MPS for DTS service.",
	///  "type": "string",
	///  "enum": [
	///    "DISABLE_MPS_FOR_DTS",
	///    "ENABLE_MPS_FOR_DTS",
	///    "AUTHORIZE_AND_ENABLE_MPS_FOR_DTS"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum MpsAction {
		#[default]
		#[serde(rename = "DISABLE_MPS_FOR_DTS")]
		DisableMpsForDts,
		#[serde(rename = "ENABLE_MPS_FOR_DTS")]
		EnableMpsForDts,
		#[serde(rename = "AUTHORIZE_AND_ENABLE_MPS_FOR_DTS")]
		AuthorizeAndEnableMpsForDts,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MpsAction> for MpsAction {
		fn from(value: &MpsAction) -> Self {
			value.clone()
		}
	}

	impl ToString for MpsAction {
		fn to_string(&self) -> String {
			match *self {
				Self::DisableMpsForDts => "DISABLE_MPS_FOR_DTS".to_string(),
				Self::EnableMpsForDts => "ENABLE_MPS_FOR_DTS".to_string(),
				Self::AuthorizeAndEnableMpsForDts => "AUTHORIZE_AND_ENABLE_MPS_FOR_DTS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MpsAction {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DISABLE_MPS_FOR_DTS" => Ok(Self::DisableMpsForDts),
				"ENABLE_MPS_FOR_DTS" => Ok(Self::EnableMpsForDts),
				"AUTHORIZE_AND_ENABLE_MPS_FOR_DTS" => Ok(Self::AuthorizeAndEnableMpsForDts),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MpsAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MpsAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MpsAction {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates whether the service data flow, corresponding to the service
	/// data flow template, is allowed or not allowed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the service data flow, corresponding
	/// to the service data flow template, is allowed or not allowed.",
	///  "type": "string",
	///  "enum": [
	///    "ALLOWED",
	///    "NOT_ALLOWED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum MulticastAccessControl {
		#[default]
		#[serde(rename = "ALLOWED")]
		Allowed,
		#[serde(rename = "NOT_ALLOWED")]
		NotAllowed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MulticastAccessControl> for MulticastAccessControl {
		fn from(value: &MulticastAccessControl) -> Self {
			value.clone()
		}
	}

	impl ToString for MulticastAccessControl {
		fn to_string(&self) -> String {
			match *self {
				Self::Allowed => "ALLOWED".to_string(),
				Self::NotAllowed => "NOT_ALLOWED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MulticastAccessControl {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ALLOWED" => Ok(Self::Allowed),
				"NOT_ALLOWED" => Ok(Self::NotAllowed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MulticastAccessControl {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MulticastAccessControl {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MulticastAccessControl {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Possible values are
	/// - ANR_NOT_SUPPORTED: Indicates that the access network does not support
	///   the report of access network information.
	/// - TZR_NOT_SUPPORTED: Indicates that the access network does not support
	///   the report of UE time zone.
	/// - LOC_NOT_SUPPORTED: Indicates that the access network does not support
	///   the report of UE Location (or PLMN Id).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- ANR_NOT_SUPPORTED: Indicates
	/// that the access network does not support the report of access network
	/// information.\n- TZR_NOT_SUPPORTED: Indicates that the access network
	/// does not support the report of UE time zone.\n- LOC_NOT_SUPPORTED:
	/// Indicates that the access network does not support the report of UE
	/// Location (or PLMN Id).\n",
	///  "type": "string",
	///  "enum": [
	///    "ANR_NOT_SUPPORTED",
	///    "TZR_NOT_SUPPORTED",
	///    "LOC_NOT_SUPPORTED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum NetLocAccessSupport {
		#[default]
		#[serde(rename = "ANR_NOT_SUPPORTED")]
		AnrNotSupported,
		#[serde(rename = "TZR_NOT_SUPPORTED")]
		TzrNotSupported,
		#[serde(rename = "LOC_NOT_SUPPORTED")]
		LocNotSupported,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NetLocAccessSupport> for NetLocAccessSupport {
		fn from(value: &NetLocAccessSupport) -> Self {
			value.clone()
		}
	}

	impl ToString for NetLocAccessSupport {
		fn to_string(&self) -> String {
			match *self {
				Self::AnrNotSupported => "ANR_NOT_SUPPORTED".to_string(),
				Self::TzrNotSupported => "TZR_NOT_SUPPORTED".to_string(),
				Self::LocNotSupported => "LOC_NOT_SUPPORTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NetLocAccessSupport {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ANR_NOT_SUPPORTED" => Ok(Self::AnrNotSupported),
				"TZR_NOT_SUPPORTED" => Ok(Self::TzrNotSupported),
				"LOC_NOT_SUPPORTED" => Ok(Self::LocNotSupported),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NetLocAccessSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NetLocAccessSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NetLocAccessSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Describes a BDT notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes a BDT notification.",
	///  "type": "object",
	///  "required": [
	///    "bdtRefId"
	///  ],
	///  "properties": {
	///    "bdtRefId": {
	///      "$ref": "#/components/schemas/BdtReferenceId"
	///    },
	///    "candPolicies": {
	///      "description": "Contains a list of the candidate transfer policies
	/// from which the AF may select a new transfer policy due to a network
	/// performance is below the criteria set by the operator.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransferPolicy"
	///      },
	///      "minItems": 1
	///    },
	///    "nwAreaInfo": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "timeWindow": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Notification {
		#[serde(rename = "bdtRefId")]
		pub bdt_ref_id: BdtReferenceId,
		/// Contains a list of the candidate transfer policies from which the AF
		/// may select a new transfer policy due to a network performance is
		/// below the criteria set by the operator.
		#[serde(
			rename = "candPolicies",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub cand_policies: Vec<TransferPolicy>,
		#[serde(
			rename = "nwAreaInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nw_area_info: Option<NetworkAreaInfo>,
		#[serde(
			rename = "timeWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_window: Option<TimeWindow>,
	}

	impl From<&Notification> for Notification {
		fn from(value: &Notification) -> Self {
			value.clone()
		}
	}

	/// Indicates that the notification of DDD Status is requested and/or that
	/// the notification of DDN Failure is requested.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates that the notification of DDD Status is requested and/or that the notification of DDN Failure is requested.",
	///  "type": "string",
	///  "enum": [
	///    "DDN_FAILURE",
	///    "DDD_STATUS"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum NotificationControlIndication {
		#[default]
		#[serde(rename = "DDN_FAILURE")]
		DdnFailure,
		#[serde(rename = "DDD_STATUS")]
		DddStatus,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationControlIndication> for NotificationControlIndication {
		fn from(value: &NotificationControlIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationControlIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::DdnFailure => "DDN_FAILURE".to_string(),
				Self::DddStatus => "DDD_STATUS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationControlIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DDN_FAILURE" => Ok(Self::DdnFailure),
				"DDD_STATUS" => Ok(Self::DddStatus),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationControlIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationControlIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationControlIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Indicates the list of Analytic ID(s) per NWDAF instance ID used for the
	/// PDU Session consumed by the SMF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the list of Analytic ID(s) per NWDAF instance
	/// ID used for the PDU Session consumed by the SMF.",
	///  "type": "object",
	///  "required": [
	///    "nwdafInstanceId"
	///  ],
	///  "properties": {
	///    "nwdafEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "nwdafInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NwdafData {
		#[serde(rename = "nwdafEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_events: Vec<NwdafEvent>,
		#[serde(rename = "nwdafInstanceId")]
		pub nwdaf_instance_id: NfInstanceId,
	}

	impl From<&NwdafData> for NwdafData {
		fn from(value: &NwdafData) -> Self {
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

	/// Indicates the SDFs without available credit and the corresponding
	/// termination action.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the SDFs without available credit and the
	/// corresponding termination action.",
	///  "type": "object",
	///  "required": [
	///    "finUnitAct"
	///  ],
	///  "properties": {
	///    "finUnitAct": {
	///      "$ref": "#/components/schemas/FinalUnitAction"
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
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
	pub struct OutOfCreditInformation {
		#[serde(rename = "finUnitAct")]
		pub fin_unit_act: FinalUnitAction,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub flows: Vec<Flows>,
	}

	impl From<&OutOfCreditInformation> for OutOfCreditInformation {
		fn from(value: &OutOfCreditInformation) -> Self {
			value.clone()
		}
	}

	/// Defines a packet filter for an IP flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Defines a packet filter for an IP flow.",
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
	pub struct PacketFilterContent(pub String);

	impl ::std::ops::Deref for PacketFilterContent {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PacketFilterContent> for String {
		fn from(value: PacketFilterContent) -> Self {
			value.0
		}
	}

	impl From<&PacketFilterContent> for PacketFilterContent {
		fn from(value: &PacketFilterContent) -> Self {
			value.clone()
		}
	}

	impl From<String> for PacketFilterContent {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PacketFilterContent {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for PacketFilterContent {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the information from a single packet filter sent from the SMF
	/// to the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the information from a single packet filter
	/// sent from the SMF to the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "flowDirection": {
	///      "$ref": "#/components/schemas/FlowDirection"
	///    },
	///    "flowLabel": {
	///      "description": "The Ipv6 flow label header field.",
	///      "type": "string"
	///    },
	///    "packFiltCont": {
	///      "$ref": "#/components/schemas/PacketFilterContent"
	///    },
	///    "packFiltId": {
	///      "description": "An identifier of packet filter.",
	///      "type": "string"
	///    },
	///    "spi": {
	///      "description": "The security parameter index of the IPSec packet.",
	///      "type": "string"
	///    },
	///    "tosTrafficClass": {
	///      "description": "Contains the Ipv4 Type-of-Service and mask field or
	/// the Ipv6 Traffic-Class field and mask field.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketFilterInfo {
		#[serde(
			rename = "flowDirection",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub flow_direction: Option<FlowDirection>,
		/// The Ipv6 flow label header field.
		#[serde(rename = "flowLabel", default, skip_serializing_if = "Option::is_none")]
		pub flow_label: Option<String>,
		#[serde(
			rename = "packFiltCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pack_filt_cont: Option<PacketFilterContent>,
		/// An identifier of packet filter.
		#[serde(
			rename = "packFiltId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pack_filt_id: Option<String>,
		/// The security parameter index of the IPSec packet.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub spi: Option<String>,
		/// Contains the Ipv4 Type-of-Service and mask field or the Ipv6
		/// Traffic-Class field and mask field.
		#[serde(
			rename = "tosTrafficClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tos_traffic_class: Option<String>,
	}

	impl From<&PacketFilterInfo> for PacketFilterInfo {
		fn from(value: &PacketFilterInfo) -> Self {
			value.clone()
		}
	}

	/// Includes the information reported by the SMF when some of the PCC rules
	/// and/or session rules are not successfully installed/activated.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Includes the information reported by the SMF when some
	/// of the PCC rules and/or session rules are not successfully
	/// installed/activated.",
	///  "type": "object",
	///  "required": [
	///    "failureCause"
	///  ],
	///  "properties": {
	///    "failureCause": {
	///      "$ref": "#/components/schemas/FailureCause"
	///    },
	///    "invalidPolicyDecs": {
	///      "description": "Indicates the invalid parameters for the reported
	/// type(s) of the failed policy decision and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InvalidParam"
	///      },
	///      "minItems": 1
	///    },
	///    "policyDecFailureReports": {
	///      "description": "Contains the type(s) of failed policy decision
	/// and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyDecisionFailureCode"
	///      },
	///      "minItems": 1
	///    },
	///    "ruleReports": {
	///      "description": "Information about the PCC rules provisioned by the
	/// PCF not successfully installed/activated.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RuleReport"
	///      },
	///      "minItems": 1
	///    },
	///    "sessRuleReports": {
	///      "description": "Information about the session rules provisioned by
	/// the PCF not successfully installed.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SessionRuleReport"
	///      },
	///      "minItems": 1
	///    },
	///    "ueCampingRep": {
	///      "$ref": "#/components/schemas/UeCampingRep"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PartialSuccessReport {
		#[serde(rename = "failureCause")]
		pub failure_cause: FailureCause,
		/// Indicates the invalid parameters for the reported type(s) of the
		/// failed policy decision and/or condition data.
		#[serde(
			rename = "invalidPolicyDecs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_policy_decs: Vec<InvalidParam>,
		/// Contains the type(s) of failed policy decision and/or condition
		/// data.
		#[serde(
			rename = "policyDecFailureReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub policy_dec_failure_reports: Vec<PolicyDecisionFailureCode>,
		/// Information about the PCC rules provisioned by the PCF not
		/// successfully installed/activated.
		#[serde(rename = "ruleReports", default, skip_serializing_if = "Vec::is_empty")]
		pub rule_reports: Vec<RuleReport>,
		/// Information about the session rules provisioned by the PCF not
		/// successfully installed.
		#[serde(
			rename = "sessRuleReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sess_rule_reports: Vec<SessionRuleReport>,
		#[serde(
			rename = "ueCampingRep",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_camping_rep: Option<UeCampingRep>,
	}

	impl From<&PartialSuccessReport> for PartialSuccessReport {
		fn from(value: &PartialSuccessReport) -> Self {
			value.clone()
		}
	}

	/// Describes the updates in authorization data of an Individual BDT Policy
	/// created by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the updates in authorization data of an
	/// Individual BDT Policy created by the PCF.\n",
	///  "type": "object",
	///  "properties": {
	///    "bdtPolData": {
	///      "$ref": "#/components/schemas/BdtPolicyDataPatch"
	///    },
	///    "bdtReqData": {
	///      "$ref": "#/components/schemas/BdtReqDataPatch"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PatchBdtPolicy {
		#[serde(
			rename = "bdtPolData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub bdt_pol_data: Option<BdtPolicyDataPatch>,
		#[serde(
			rename = "bdtReqData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub bdt_req_data: Option<BdtReqDataPatch>,
	}

	impl From<&PatchBdtPolicy> for PatchBdtPolicy {
		fn from(value: &PatchBdtPolicy) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - LTE_PC5: This value is used to indicate that UE supports PC5 LTE RAT
	///   for V2X communications
	///  over the PC5 reference point.
	/// - NR_PC5: This value is used to indicate that UE supports PC5 NR RAT for
	///   V2X communications
	///  over the PC5 reference point.
	/// - LTE_NR_PC5: This value is used to indicate that UE supports both PC5
	///   LTE and NR RAT for
	///  V2X communications over the PC5 reference point.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- LTE_PC5: This value is used to
	/// indicate that UE supports PC5 LTE RAT for V2X communications\n  over the
	/// PC5 reference point.\n- NR_PC5: This value is used to indicate that UE
	/// supports PC5 NR RAT for V2X communications\n  over the PC5 reference
	/// point.\n- LTE_NR_PC5: This value is used to indicate that UE supports
	/// both PC5 LTE and NR RAT for\n  V2X communications over the PC5 reference
	/// point. \n",
	///  "type": "string",
	///  "enum": [
	///    "LTE_PC5",
	///    "NR_PC5",
	///    "LTE_NR_PC5"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum Pc5Capability {
		#[default]
		#[serde(rename = "LTE_PC5")]
		LtePc5,
		#[serde(rename = "NR_PC5")]
		NrPc5,
		#[serde(rename = "LTE_NR_PC5")]
		LteNrPc5,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Pc5Capability> for Pc5Capability {
		fn from(value: &Pc5Capability) -> Self {
			value.clone()
		}
	}

	impl ToString for Pc5Capability {
		fn to_string(&self) -> String {
			match *self {
				Self::LtePc5 => "LTE_PC5".to_string(),
				Self::NrPc5 => "NR_PC5".to_string(),
				Self::LteNrPc5 => "LTE_NR_PC5".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Pc5Capability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LTE_PC5" => Ok(Self::LtePc5),
				"NR_PC5" => Ok(Self::NrPc5),
				"LTE_NR_PC5" => Ok(Self::LteNrPc5),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Pc5Capability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Pc5Capability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Pc5Capability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the policy control events that can be subscribed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the policy control events that can be
	/// subscribed.",
	///  "type": "string",
	///  "enum": [
	///    "AC_TY_CH",
	///    "PLMN_CH",
	///    "SAC_CH",
	///    "SAT_CATEGORY_CH",
	///    "SUCCESS_UE_POL_DEL_SP",
	///    "UNSUCCESS_UE_POL_DEL_SP"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PcEvent {
		#[default]
		#[serde(rename = "AC_TY_CH")]
		AcTyCh,
		#[serde(rename = "PLMN_CH")]
		PlmnCh,
		#[serde(rename = "SAC_CH")]
		SacCh,
		#[serde(rename = "SAT_CATEGORY_CH")]
		SatCategoryCh,
		#[serde(rename = "SUCCESS_UE_POL_DEL_SP")]
		SuccessUePolDelSp,
		#[serde(rename = "UNSUCCESS_UE_POL_DEL_SP")]
		UnsuccessUePolDelSp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PcEvent> for PcEvent {
		fn from(value: &PcEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for PcEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::AcTyCh => "AC_TY_CH".to_string(),
				Self::PlmnCh => "PLMN_CH".to_string(),
				Self::SacCh => "SAC_CH".to_string(),
				Self::SatCategoryCh => "SAT_CATEGORY_CH".to_string(),
				Self::SuccessUePolDelSp => "SUCCESS_UE_POL_DEL_SP".to_string(),
				Self::UnsuccessUePolDelSp => "UNSUCCESS_UE_POL_DEL_SP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PcEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AC_TY_CH" => Ok(Self::AcTyCh),
				"PLMN_CH" => Ok(Self::PlmnCh),
				"SAC_CH" => Ok(Self::SacCh),
				"SAT_CATEGORY_CH" => Ok(Self::SatCategoryCh),
				"SUCCESS_UE_POL_DEL_SP" => Ok(Self::SuccessUePolDelSp),
				"UNSUCCESS_UE_POL_DEL_SP" => Ok(Self::UnsuccessUePolDelSp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PcEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PcEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PcEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents notifications about Policy Control events related to an
	/// Individual Policy Events Subscription resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents notifications about Policy Control events
	/// related to an Individual Policy Events Subscription resource.\n",
	///  "type": "object",
	///  "required": [
	///    "eventNotifs",
	///    "notifId"
	///  ],
	///  "properties": {
	///    "eventNotifs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PcEventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "notifId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PcEventExposureNotif {
		#[serde(rename = "eventNotifs")]
		pub event_notifs: Vec<PcEventNotification>,
		#[serde(rename = "notifId")]
		pub notif_id: String,
	}

	impl From<&PcEventExposureNotif> for PcEventExposureNotif {
		fn from(value: &PcEventExposureNotif) -> Self {
			value.clone()
		}
	}

	/// Represents an Individual Policy Events Subscription resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual Policy Events Subscription
	/// resource.",
	///  "type": "object",
	///  "required": [
	///    "eventSubs",
	///    "notifId",
	///    "notifUri"
	///  ],
	///  "properties": {
	///    "eventNotifs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PcEventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "eventSubs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PcEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "eventsRepInfo": {
	///      "$ref": "#/components/schemas/ReportingInformation"
	///    },
	///    "filterDnns": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "filterServices": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceIdentification"
	///      },
	///      "minItems": 1
	///    },
	///    "filterSnssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "snssaiDnns": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiDnnCombination"
	///      },
	///      "minItems": 1
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PcEventExposureSubsc {
		#[serde(rename = "eventNotifs", default, skip_serializing_if = "Vec::is_empty")]
		pub event_notifs: Vec<PcEventNotification>,
		#[serde(rename = "eventSubs")]
		pub event_subs: Vec<PcEvent>,
		#[serde(
			rename = "eventsRepInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub events_rep_info: Option<ReportingInformation>,
		#[serde(rename = "filterDnns", default, skip_serializing_if = "Vec::is_empty")]
		pub filter_dnns: Vec<Dnn>,
		#[serde(
			rename = "filterServices",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub filter_services: Vec<ServiceIdentification>,
		#[serde(
			rename = "filterSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub filter_snssais: Vec<Snssai>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<GroupId>,
		#[serde(rename = "notifId")]
		pub notif_id: String,
		#[serde(rename = "notifUri")]
		pub notif_uri: Uri,
		#[serde(rename = "snssaiDnns", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_dnns: Vec<SnssaiDnnCombination>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
	}

	impl From<&PcEventExposureSubsc> for PcEventExposureSubsc {
		fn from(value: &PcEventExposureSubsc) -> Self {
			value.clone()
		}
	}

	/// Represents the information reported for a Policy Control event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the information reported for a Policy
	/// Control event.",
	///  "type": "object",
	///  "required": [
	///    "event",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "accType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "addAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "anGwAddr": {
	///      "$ref": "#/components/schemas/AnGwAddress"
	///    },
	///    "appliedCov": {
	///      "$ref": "#/components/schemas/schemas-ServiceAreaCoverageInfo"
	///    },
	///    "delivFailure": {
	///      "$ref": "#/components/schemas/Failure"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/PcEvent"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "pduSessionInfo": {
	///      "$ref": "#/components/schemas/PduSessionInformation"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "relAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "repServices": {
	///      "$ref": "#/components/schemas/ServiceIdentification"
	///    },
	///    "satBackhaulCategory": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
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
	pub struct PcEventNotification {
		#[serde(rename = "accType", default, skip_serializing_if = "Option::is_none")]
		pub acc_type: Option<AccessType>,
		#[serde(
			rename = "addAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_access_info: Option<AdditionalAccessInfo>,
		#[serde(rename = "anGwAddr", default, skip_serializing_if = "Option::is_none")]
		pub an_gw_addr: Option<AnGwAddress>,
		#[serde(
			rename = "appliedCov",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub applied_cov: Option<SchemasServiceAreaCoverageInfo>,
		#[serde(
			rename = "delivFailure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub deliv_failure: Option<Failure>,
		pub event: PcEvent,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "pduSessionInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_info: Option<PduSessionInformation>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnIdNid>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "relAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rel_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "repServices",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_services: Option<ServiceIdentification>,
		#[serde(
			rename = "satBackhaulCategory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sat_backhaul_category: Option<SatelliteBackhaulCategory>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
	}

	impl From<&PcEventNotification> for PcEventNotification {
		fn from(value: &PcEventNotification) -> Self {
			value.clone()
		}
	}

	/// Contains a PCC rule information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a PCC rule information.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "pccRuleId"
	///  ],
	///  "properties": {
	///    "addrPreserInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "afSigProtocol": {
	///      "$ref": "#/components/schemas/AfSigProtocol"
	///    },
	///    "appDescriptor": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "appId": {
	///      "description": "A reference to the application detection filter
	/// configured at the UPF.",
	///      "type": "string"
	///    },
	///    "appReloc": {
	///      "description": "Indication of application relocation possibility.",
	///      "type": "boolean"
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "ddNotifCtrl": {
	///      "$ref": "#/components/schemas/DownlinkDataNotificationControl"
	///    },
	///    "ddNotifCtrl2": {
	///      "$ref": "#/components/schemas/DownlinkDataNotificationControlRm"
	///    },
	///    "disUeNotif": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "easRedisInd": {
	///      "description": "Indicates the EAS rediscovery is required.",
	///      "type": "boolean"
	///    },
	///    "flowInfos": {
	///      "description": "An array of IP flow packet filter information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "packFiltAllPrec": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "pccRuleId": {
	///      "description": "Univocally identifies the PCC rule within a PDU
	/// session.",
	///      "type": "string"
	///    },
	///    "precedence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "refAltQosParams": {
	///      "description": "A Reference to the QosData policy decision type for
	/// the Alternative QoS parameter sets of the service data flow.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "refChgData": {
	///      "description": "A reference to the ChargingData policy decision
	/// type. It is the chgId described in clause 5.6.2.11.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refChgN3gData": {
	///      "description": "A reference to the ChargingData policy decision
	/// type only applicable to Non-3GPP access if \"ATSSS\" feature is
	/// supported. It is the chgId described in clause 5.6.2.11.\n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refCondData": {
	///      "description": "A reference to the condition data. It is the condId
	/// described in clause 5.6.2.9.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refQosData": {
	///      "description": "A reference to the QosData policy decision type. It
	/// is the qosId described in clause 5.6.2.8.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refQosMon": {
	///      "description": "A reference to the QosMonitoringData policy
	/// decision type. It is the qmId described in clause 5.6.2.40.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refTcData": {
	///      "description": "A reference to the TrafficControlData policy
	/// decision type. It is the tcId described in clause 5.6.2.10.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refUmData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type. It is the umId described in clause 5.6.2.12.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refUmN3gData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type only applicable to Non-3GPP access if \"ATSSS\" feature is
	/// supported. It is the umId described in clause 5.6.2.12. \n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "tscaiInputDl": {
	///      "$ref": "#/components/schemas/schemas-TscaiInputContainer"
	///    },
	///    "tscaiInputUl": {
	///      "$ref": "#/components/schemas/schemas-TscaiInputContainer"
	///    },
	///    "tscaiTimeDom": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PccRule(pub Option<PccRuleInner>);

	impl ::std::ops::Deref for PccRule {
		type Target = Option<PccRuleInner>;
		fn deref(&self) -> &Option<PccRuleInner> {
			&self.0
		}
	}

	impl From<PccRule> for Option<PccRuleInner> {
		fn from(value: PccRule) -> Self {
			value.0
		}
	}

	impl From<&PccRule> for PccRule {
		fn from(value: &PccRule) -> Self {
			value.clone()
		}
	}

	impl From<Option<PccRuleInner>> for PccRule {
		fn from(value: Option<PccRuleInner>) -> Self {
			Self(value)
		}
	}

	/// Contains a PCC rule information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a PCC rule information.",
	///  "type": "object",
	///  "required": [
	///    "pccRuleId"
	///  ],
	///  "properties": {
	///    "addrPreserInd": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "afSigProtocol": {
	///      "$ref": "#/components/schemas/AfSigProtocol"
	///    },
	///    "appDescriptor": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "appId": {
	///      "description": "A reference to the application detection filter
	/// configured at the UPF.",
	///      "type": "string"
	///    },
	///    "appReloc": {
	///      "description": "Indication of application relocation possibility.",
	///      "type": "boolean"
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "ddNotifCtrl": {
	///      "$ref": "#/components/schemas/DownlinkDataNotificationControl"
	///    },
	///    "ddNotifCtrl2": {
	///      "$ref": "#/components/schemas/DownlinkDataNotificationControlRm"
	///    },
	///    "disUeNotif": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "easRedisInd": {
	///      "description": "Indicates the EAS rediscovery is required.",
	///      "type": "boolean"
	///    },
	///    "flowInfos": {
	///      "description": "An array of IP flow packet filter information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "packFiltAllPrec": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "pccRuleId": {
	///      "description": "Univocally identifies the PCC rule within a PDU
	/// session.",
	///      "type": "string"
	///    },
	///    "precedence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "refAltQosParams": {
	///      "description": "A Reference to the QosData policy decision type for
	/// the Alternative QoS parameter sets of the service data flow.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "refChgData": {
	///      "description": "A reference to the ChargingData policy decision
	/// type. It is the chgId described in clause 5.6.2.11.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refChgN3gData": {
	///      "description": "A reference to the ChargingData policy decision
	/// type only applicable to Non-3GPP access if \"ATSSS\" feature is
	/// supported. It is the chgId described in clause 5.6.2.11.\n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refCondData": {
	///      "description": "A reference to the condition data. It is the condId
	/// described in clause 5.6.2.9.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refQosData": {
	///      "description": "A reference to the QosData policy decision type. It
	/// is the qosId described in clause 5.6.2.8.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refQosMon": {
	///      "description": "A reference to the QosMonitoringData policy
	/// decision type. It is the qmId described in clause 5.6.2.40.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refTcData": {
	///      "description": "A reference to the TrafficControlData policy
	/// decision type. It is the tcId described in clause 5.6.2.10.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refUmData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type. It is the umId described in clause 5.6.2.12.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "refUmN3gData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type only applicable to Non-3GPP access if \"ATSSS\" feature is
	/// supported. It is the umId described in clause 5.6.2.12. \n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 1,
	///      "minItems": 1
	///    },
	///    "tscaiInputDl": {
	///      "$ref": "#/components/schemas/schemas-TscaiInputContainer"
	///    },
	///    "tscaiInputUl": {
	///      "$ref": "#/components/schemas/schemas-TscaiInputContainer"
	///    },
	///    "tscaiTimeDom": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PccRuleInner {
		#[serde(
			rename = "addrPreserInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub addr_preser_ind: Option<bool>,
		#[serde(
			rename = "afSigProtocol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_sig_protocol: Option<AfSigProtocol>,
		#[serde(
			rename = "appDescriptor",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub app_descriptor: Option<Bytes>,
		/// A reference to the application detection filter configured at the
		/// UPF.
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<String>,
		/// Indication of application relocation possibility.
		#[serde(rename = "appReloc", default, skip_serializing_if = "Option::is_none")]
		pub app_reloc: Option<bool>,
		#[serde(rename = "contVer", default, skip_serializing_if = "Option::is_none")]
		pub cont_ver: Option<ContentVersion>,
		#[serde(
			rename = "ddNotifCtrl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dd_notif_ctrl: Option<DownlinkDataNotificationControl>,
		#[serde(
			rename = "ddNotifCtrl2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dd_notif_ctrl2: Option<DownlinkDataNotificationControlRm>,
		#[serde(
			rename = "disUeNotif",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dis_ue_notif: Option<bool>,
		/// Indicates the EAS rediscovery is required.
		#[serde(
			rename = "easRedisInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eas_redis_ind: Option<bool>,
		/// An array of IP flow packet filter information.
		#[serde(rename = "flowInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub flow_infos: Vec<FlowInformation>,
		#[serde(
			rename = "packFiltAllPrec",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pack_filt_all_prec: Option<Uinteger>,
		/// Univocally identifies the PCC rule within a PDU session.
		#[serde(rename = "pccRuleId")]
		pub pcc_rule_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub precedence: Option<Uinteger>,
		/// A Reference to the QosData policy decision type for the Alternative
		/// QoS parameter sets of the service data flow.
		#[serde(
			rename = "refAltQosParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ref_alt_qos_params: Vec<String>,
		/// A reference to the ChargingData policy decision type. It is the
		/// chgId described in clause 5.6.2.11.
		#[serde(
			rename = "refChgData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_chg_data: Option<[String; 1usize]>,
		/// A reference to the ChargingData policy decision type only applicable
		/// to Non-3GPP access if "ATSSS" feature is supported. It is the chgId
		/// described in clause 5.6.2.11.
		#[serde(
			rename = "refChgN3gData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_chg_n3g_data: Option<[String; 1usize]>,
		/// A reference to the condition data. It is the condId described in
		/// clause 5.6.2.9.
		#[serde(
			rename = "refCondData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_cond_data: Option<String>,
		/// A reference to the QosData policy decision type. It is the qosId
		/// described in clause 5.6.2.8.
		#[serde(
			rename = "refQosData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_qos_data: Option<[String; 1usize]>,
		/// A reference to the QosMonitoringData policy decision type. It is the
		/// qmId described in clause 5.6.2.40.
		#[serde(rename = "refQosMon", default, skip_serializing_if = "Option::is_none")]
		pub ref_qos_mon: Option<[String; 1usize]>,
		/// A reference to the TrafficControlData policy decision type. It is
		/// the tcId described in clause 5.6.2.10.
		#[serde(rename = "refTcData", default, skip_serializing_if = "Option::is_none")]
		pub ref_tc_data: Option<[String; 1usize]>,
		/// A reference to UsageMonitoringData policy decision type. It is the
		/// umId described in clause 5.6.2.12.
		#[serde(rename = "refUmData", default, skip_serializing_if = "Option::is_none")]
		pub ref_um_data: Option<[String; 1usize]>,
		/// A reference to UsageMonitoringData policy decision type only
		/// applicable to Non-3GPP access if "ATSSS" feature is supported. It is
		/// the umId described in clause 5.6.2.12.
		#[serde(
			rename = "refUmN3gData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_um_n3g_data: Option<[String; 1usize]>,
		#[serde(
			rename = "tscaiInputDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_dl: Option<SchemasTscaiInputContainer>,
		#[serde(
			rename = "tscaiInputUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_input_ul: Option<SchemasTscaiInputContainer>,
		#[serde(
			rename = "tscaiTimeDom",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tscai_time_dom: Option<Uinteger>,
	}

	impl From<&PccRuleInner> for PccRuleInner {
		fn from(value: &PccRuleInner) -> Self {
			value.clone()
		}
	}

	/// Contains PCF address information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains PCF address information.",
	///  "type": "object",
	///  "properties": {
	///    "bindingInfo": {
	///      "description": "contains the binding indications of the PCF.",
	///      "type": "string"
	///    },
	///    "pcfFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "pcfIpEndPoints": {
	///      "description": "IP end points of the PCF hosting the
	/// Npcf_PolicyAuthorization service.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpEndPoint"
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
	pub struct PcfAddressingInfo {
		/// contains the binding indications of the PCF.
		#[serde(
			rename = "bindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub binding_info: Option<String>,
		#[serde(rename = "pcfFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pcf_fqdn: Option<Fqdn>,
		/// IP end points of the PCF hosting the Npcf_PolicyAuthorization
		/// service.
		#[serde(
			rename = "pcfIpEndPoints",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pcf_ip_end_points: Vec<IpEndPoint>,
	}

	impl From<&PcfAddressingInfo> for PcfAddressingInfo {
		fn from(value: &PcfAddressingInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates P-CSCF restoration.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates P-CSCF restoration.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ueIpv4"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ueIpv6"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ipDomain": {
	///      "type": "string"
	///    },
	///    "sliceInfo": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "ueIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6": {
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
	pub enum PcscfRestorationRequestData {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			#[serde(rename = "sliceInfo", default, skip_serializing_if = "Option::is_none")]
			slice_info: Option<Snssai>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(rename = "ueIpv4")]
			ue_ipv4: Ipv4Addr,
		},
		Variant1 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			#[serde(rename = "sliceInfo", default, skip_serializing_if = "Option::is_none")]
			slice_info: Option<Snssai>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(rename = "ueIpv6")]
			ue_ipv6: Ipv6Addr,
		},
	}

	impl From<&PcscfRestorationRequestData> for PcscfRestorationRequestData {
		fn from(value: &PcscfRestorationRequestData) -> Self {
			value.clone()
		}
	}

	/// Indicates PDU session information for the concerned
	/// established/terminated PDU session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates PDU session information for the concerned
	/// established/terminated PDU session.",
	///  "type": "object",
	///  "required": [
	///    "evNotif"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "evNotif": {
	///      "$ref": "#/components/schemas/AfEventNotification"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "pcfInfo": {
	///      "$ref": "#/components/schemas/PcfAddressingInfo"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "status": {
	///      "$ref": "#/components/schemas/PduSessionStatus"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "ueIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "ueMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionEventNotification {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "evNotif")]
		pub ev_notif: AfEventNotification,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "pcfInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_info: Option<PcfAddressingInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<PduSessionStatus>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(rename = "ueIpv4", default, skip_serializing_if = "Option::is_none")]
		pub ue_ipv4: Option<Ipv4Addr>,
		#[serde(rename = "ueIpv6", default, skip_serializing_if = "Option::is_none")]
		pub ue_ipv6: Option<Ipv6Addr>,
		#[serde(rename = "ueMac", default, skip_serializing_if = "Option::is_none")]
		pub ue_mac: Option<MacAddr48>,
	}

	impl From<&PduSessionEventNotification> for PduSessionEventNotification {
		fn from(value: &PduSessionEventNotification) -> Self {
			value.clone()
		}
	}

	/// Represents PDU session identification information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents PDU session identification information.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ueMac"
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "ueIpv4"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "ueIpv6"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "dnn",
	///    "snssai"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ipDomain": {
	///      "type": "string"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "ueIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "ueMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum PduSessionInformation {
		#[default]
		Variant0 {
			dnn: Dnn,
			#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
			ip_domain: Option<String>,
			snssai: Snssai,
			#[serde(rename = "ueMac")]
			ue_mac: MacAddr48,
		},
		Variant1(PduSessionInformationVariant1),
	}

	impl From<&PduSessionInformation> for PduSessionInformation {
		fn from(value: &PduSessionInformation) -> Self {
			value.clone()
		}
	}

	impl From<PduSessionInformationVariant1> for PduSessionInformation {
		fn from(value: PduSessionInformationVariant1) -> Self {
			Self::Variant1(value)
		}
	}

	/// PduSessionInformationVariant1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "type": "object",
	///      "required": [
	///        "dnn",
	///        "snssai"
	///      ],
	///      "properties": {
	///        "dnn": {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        "ipDomain": {
	///          "type": "string"
	///        },
	///        "snssai": {
	///          "$ref": "#/components/schemas/Snssai"
	///        },
	///        "ueIpv4": {
	///          "$ref": "#/components/schemas/Ipv4Addr"
	///        },
	///        "ueIpv6": {
	///          "$ref": "#/components/schemas/Ipv6Prefix"
	///        },
	///        "ueMac": {
	///          "$ref": "#/components/schemas/MacAddr48"
	///        }
	///      }
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "ueIpv4"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "ueIpv6"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "ueMac"
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
	pub enum PduSessionInformationVariant1 {
		#[default]
		Variant0(PduSessionInformationVariant1Variant0),
		Variant1(PduSessionInformationVariant1Variant1),
	}

	impl From<&PduSessionInformationVariant1> for PduSessionInformationVariant1 {
		fn from(value: &PduSessionInformationVariant1) -> Self {
			value.clone()
		}
	}

	impl From<PduSessionInformationVariant1Variant0> for PduSessionInformationVariant1 {
		fn from(value: PduSessionInformationVariant1Variant0) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<PduSessionInformationVariant1Variant1> for PduSessionInformationVariant1 {
		fn from(value: PduSessionInformationVariant1Variant1) -> Self {
			Self::Variant1(value)
		}
	}

	/// PduSessionInformationVariant1Variant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {},
	///    {
	///      "allOf": [
	///        {
	///          "type": "object",
	///          "required": [
	///            "dnn",
	///            "snssai"
	///          ],
	///          "properties": {
	///            "dnn": {
	///              "$ref": "#/components/schemas/Dnn"
	///            },
	///            "ipDomain": {
	///              "type": "string"
	///            },
	///            "snssai": {
	///              "$ref": "#/components/schemas/Snssai"
	///            },
	///            "ueIpv4": {
	///              "$ref": "#/components/schemas/Ipv4Addr"
	///            },
	///            "ueIpv6": {
	///              "$ref": "#/components/schemas/Ipv6Prefix"
	///            },
	///            "ueMac": {
	///              "$ref": "#/components/schemas/MacAddr48"
	///            }
	///          }
	///        },
	///        {
	///          "required": [
	///            "ueIpv4"
	///          ]
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "ueIpv6"
	///            ]
	///          }
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "allOf": [
	///          {
	///            "type": "object",
	///            "required": [
	///              "dnn",
	///              "snssai"
	///            ],
	///            "properties": {
	///              "dnn": {
	///                "$ref": "#/components/schemas/Dnn"
	///              },
	///              "ipDomain": {
	///                "type": "string"
	///              },
	///              "snssai": {
	///                "$ref": "#/components/schemas/Snssai"
	///              },
	///              "ueIpv4": {
	///                "$ref": "#/components/schemas/Ipv4Addr"
	///              },
	///              "ueIpv6": {
	///                "$ref": "#/components/schemas/Ipv6Prefix"
	///              },
	///              "ueMac": {
	///                "$ref": "#/components/schemas/MacAddr48"
	///              }
	///            }
	///          },
	///          {
	///            "required": [
	///              "ueIpv6"
	///            ]
	///          },
	///          {
	///            "not": {
	///              "required": [
	///                "ueIpv4"
	///              ]
	///            }
	///          }
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
	pub enum PduSessionInformationVariant1Variant0 {
		#[default]
		None,
	}

	impl From<&PduSessionInformationVariant1Variant0> for PduSessionInformationVariant1Variant0 {
		fn from(value: &PduSessionInformationVariant1Variant0) -> Self {
			value.clone()
		}
	}

	/// PduSessionInformationVariant1Variant1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {},
	///    {
	///      "allOf": [
	///        {
	///          "type": "object",
	///          "required": [
	///            "dnn",
	///            "snssai"
	///          ],
	///          "properties": {
	///            "dnn": {
	///              "$ref": "#/components/schemas/Dnn"
	///            },
	///            "ipDomain": {
	///              "type": "string"
	///            },
	///            "snssai": {
	///              "$ref": "#/components/schemas/Snssai"
	///            },
	///            "ueIpv4": {
	///              "$ref": "#/components/schemas/Ipv4Addr"
	///            },
	///            "ueIpv6": {
	///              "$ref": "#/components/schemas/Ipv6Prefix"
	///            },
	///            "ueMac": {
	///              "$ref": "#/components/schemas/MacAddr48"
	///            }
	///          }
	///        },
	///        {
	///          "required": [
	///            "ueIpv6"
	///          ]
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "ueIpv4"
	///            ]
	///          }
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "allOf": [
	///          {
	///            "type": "object",
	///            "required": [
	///              "dnn",
	///              "snssai"
	///            ],
	///            "properties": {
	///              "dnn": {
	///                "$ref": "#/components/schemas/Dnn"
	///              },
	///              "ipDomain": {
	///                "type": "string"
	///              },
	///              "snssai": {
	///                "$ref": "#/components/schemas/Snssai"
	///              },
	///              "ueIpv4": {
	///                "$ref": "#/components/schemas/Ipv4Addr"
	///              },
	///              "ueIpv6": {
	///                "$ref": "#/components/schemas/Ipv6Prefix"
	///              },
	///              "ueMac": {
	///                "$ref": "#/components/schemas/MacAddr48"
	///              }
	///            }
	///          },
	///          {
	///            "required": [
	///              "ueIpv4"
	///            ]
	///          },
	///          {
	///            "not": {
	///              "required": [
	///                "ueIpv6"
	///              ]
	///            }
	///          }
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
	pub enum PduSessionInformationVariant1Variant1 {
		#[default]
		None,
	}

	impl From<&PduSessionInformationVariant1Variant1> for PduSessionInformationVariant1Variant1 {
		fn from(value: &PduSessionInformationVariant1Variant1) -> Self {
			value.clone()
		}
	}

	/// Contains the SMF PDU Session release cause.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the SMF PDU Session release cause.",
	///  "type": "string",
	///  "enum": [
	///    "PS_TO_CS_HO",
	///    "RULE_ERROR"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PduSessionRelCause {
		#[default]
		#[serde(rename = "PS_TO_CS_HO")]
		PsToCsHo,
		#[serde(rename = "RULE_ERROR")]
		RuleError,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PduSessionRelCause> for PduSessionRelCause {
		fn from(value: &PduSessionRelCause) -> Self {
			value.clone()
		}
	}

	impl ToString for PduSessionRelCause {
		fn to_string(&self) -> String {
			match *self {
				Self::PsToCsHo => "PS_TO_CS_HO".to_string(),
				Self::RuleError => "RULE_ERROR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionRelCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PS_TO_CS_HO" => Ok(Self::PsToCsHo),
				"RULE_ERROR" => Ok(Self::RuleError),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionRelCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionRelCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionRelCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates whether the PDU session is established or terminated.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the PDU session is established or
	/// terminated.",
	///  "type": "string",
	///  "enum": [
	///    "ESTABLISHED",
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
	pub enum PduSessionStatus {
		#[default]
		#[serde(rename = "ESTABLISHED")]
		Established,
		#[serde(rename = "TERMINATED")]
		Terminated,
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
				Self::Established => "ESTABLISHED".to_string(),
				Self::Terminated => "TERMINATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ESTABLISHED" => Ok(Self::Established),
				"TERMINATED" => Ok(Self::Terminated),
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

	/// Contains the new TSC user plane node information and may contain the
	/// DS-TT port and/or NW-TT port management information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the new TSC user plane node information and may contain the DS-TT port and/or NW-TT port management information.",
	///  "type": "object",
	///  "required": [
	///    "tsnBridgeInfo"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ipDomain": {
	///      "description": "IPv4 address domain identifier.",
	///      "type": "string"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "tsnBridgeInfo": {
	///      "$ref": "#/components/schemas/TsnBridgeInfo"
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "ueIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6AddrPrefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionTsnBridge {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		/// IPv4 address domain identifier.
		#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
		pub ip_domain: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "tsnBridgeInfo")]
		pub tsn_bridge_info: TsnBridgeInfo,
		#[serde(
			rename = "tsnBridgeManCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_man_cont: Option<BridgeManagementContainer>,
		#[serde(
			rename = "tsnPortManContDstt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_port_man_cont_dstt: Option<PortManagementContainer>,
		#[serde(
			rename = "tsnPortManContNwtts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
		#[serde(
			rename = "ueIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "ueIpv6AddrPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv6_addr_prefix: Option<Ipv6Prefix>,
	}

	impl From<&PduSessionTsnBridge> for PduSessionTsnBridge {
		fn from(value: &PduSessionTsnBridge) -> Self {
			value.clone()
		}
	}

	/// Contains the PDUID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the PDUID.",
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
	pub struct Pduid(pub String);

	impl ::std::ops::Deref for Pduid {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Pduid> for String {
		fn from(value: Pduid) -> Self {
			value.0
		}
	}

	impl From<&Pduid> for Pduid {
		fn from(value: &Pduid) -> Self {
			value.clone()
		}
	}

	impl From<String> for Pduid {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Pduid {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Pduid {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the ProSe Discovery UE ID and its validity timer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the ProSe Discovery UE ID and its validity
	/// timer.",
	///  "type": "object",
	///  "required": [
	///    "expiry",
	///    "pduid"
	///  ],
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "pduid": {
	///      "$ref": "#/components/schemas/Pduid"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduidInformation {
		pub expiry: DateTime,
		pub pduid: Pduid,
	}

	impl From<&PduidInformation> for PduidInformation {
		fn from(value: &PduidInformation) -> Self {
			value.clone()
		}
	}

	/// Represents an individual AM Policy Association resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an individual AM Policy Association
	/// resource.",
	///  "type": "object",
	///  "required": [
	///    "suppFeat"
	///  ],
	///  "properties": {
	///    "asTimeDisParam": {
	///      "$ref": "#/components/schemas/AsTimeDistributionParam"
	///    },
	///    "matchPdus": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionInfo"
	///      }
	///    },
	///    "pcfUeInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pras": {
	///      "description": "Contains the presence reporting area(s) for which
	/// reporting was requested. The praId attribute within the PresenceInfo
	/// data type is the key of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "request": {
	///      "$ref": "#/components/schemas/PolicyAssociationRequest"
	///    },
	///    "rfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "servAreaRes": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "smfSelInfo": {
	///      "$ref": "#/components/schemas/SmfSelectionData"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetRfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the PCF subscribes.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "ueAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "ueSliceMbrs": {
	///      "description": "One or more UE-Slice-MBR(s) for S-NSSAI(s) of
	/// serving PLMN as part of the AMF Access and Mobility Policy as determined
	/// by the PCF.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeSliceMbr"
	///      },
	///      "minItems": 1
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
	pub struct PolicyAssociation {
		#[serde(
			rename = "asTimeDisParam",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub as_time_dis_param: Option<AsTimeDistributionParam>,
		#[serde(rename = "matchPdus", default, skip_serializing_if = "Option::is_none")]
		pub match_pdus: Option<Vec<PduSessionInfo>>,
		#[serde(rename = "pcfUeInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_ue_info: Option<PcfUeCallbackInfo>,
		/// Contains the presence reporting area(s) for which reporting was
		/// requested. The praId attribute within the PresenceInfo data type is
		/// the key of the map.
		#[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
		pub pras: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub request: Option<PolicyAssociationRequest>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rfsp: Option<RfspIndex>,
		#[serde(
			rename = "servAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_area_res: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "smfSelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_sel_info: Option<SmfSelectionData>,
		#[serde(rename = "suppFeat")]
		pub supp_feat: SupportedFeatures,
		#[serde(
			rename = "targetRfsp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_rfsp: Option<RfspIndex>,
		/// Request Triggers that the PCF subscribes.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<RequestTrigger>,
		#[serde(rename = "ueAmbr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ambr: Option<Ambr>,
		/// One or more UE-Slice-MBR(s) for S-NSSAI(s) of serving PLMN as part
		/// of the AMF Access and Mobility Policy as determined by the PCF.
		#[serde(rename = "ueSliceMbrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_slice_mbrs: Vec<UeSliceMbr>,
		#[serde(
			rename = "wlServAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wl_serv_area_res: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&PolicyAssociation> for PolicyAssociation {
		fn from(value: &PolicyAssociation) -> Self {
			value.clone()
		}
	}

	/// Contains the description of a policy association that is returned by the
	/// PCF when a policy Association is created, updated, or read.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the description of a policy association that
	/// is returned by the PCF when a policy Association is created, updated, or
	/// read.\n",
	///  "type": "object",
	///  "required": [
	///    "suppFeat"
	///  ],
	///  "properties": {
	///    "n2Pc5Pol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "n2Pc5ProSePol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "pras": {
	///      "description": "Contains the presence reporting area(s) for which
	/// reporting was requested. The praId attribute within the PresenceInfo
	/// data type is the key of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "request": {
	///      "$ref": "#/components/schemas/PolicyAssociationRequest1"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the PCF subscribes. Only
	/// values \"LOC_CH\" and \"PRA_CH\" are permitted.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger1"
	///      },
	///      "minItems": 1
	///    },
	///    "uePolicy": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PolicyAssociation1 {
		#[serde(rename = "n2Pc5Pol", default, skip_serializing_if = "Option::is_none")]
		pub n2_pc5_pol: Option<N2InfoContent>,
		#[serde(
			rename = "n2Pc5ProSePol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_pc5_pro_se_pol: Option<N2InfoContent>,
		/// Contains the presence reporting area(s) for which reporting was
		/// requested. The praId attribute within the PresenceInfo data type is
		/// the key of the map.
		#[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
		pub pras: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub request: Option<PolicyAssociationRequest1>,
		#[serde(rename = "suppFeat")]
		pub supp_feat: SupportedFeatures,
		/// Request Triggers that the PCF subscribes. Only values "LOC_CH" and
		/// "PRA_CH" are permitted.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<RequestTrigger1>,
		#[serde(rename = "uePolicy", default, skip_serializing_if = "Option::is_none")]
		pub ue_policy: Option<Bytes>,
	}

	impl From<&PolicyAssociation1> for PolicyAssociation1 {
		fn from(value: &PolicyAssociation1) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - UNSPECIFIED: This value is used for unspecified reasons.
	/// - UE_SUBSCRIPTION: This value is used to indicate that the session needs
	///   to be
	///  terminated because the subscription of UE has changed (e.g. was
	/// removed).
	/// - INSUFFICIENT_RES: This value is used to indicate that the server is
	///   overloaded and
	///  needs to abort the session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNSPECIFIED: This value is used
	/// for unspecified reasons.\n- UE_SUBSCRIPTION: This value is used to
	/// indicate that the session needs to be\n  terminated because the
	/// subscription of UE has changed (e.g. was removed).\n- INSUFFICIENT_RES:
	/// This value is used to indicate that the server is overloaded and\n
	/// needs to abort the session.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNSPECIFIED",
	///    "UE_SUBSCRIPTION",
	///    "INSUFFICIENT_RES"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PolicyAssociationReleaseCause {
		#[default]
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(rename = "UE_SUBSCRIPTION")]
		UeSubscription,
		#[serde(rename = "INSUFFICIENT_RES")]
		InsufficientRes,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PolicyAssociationReleaseCause> for PolicyAssociationReleaseCause {
		fn from(value: &PolicyAssociationReleaseCause) -> Self {
			value.clone()
		}
	}

	impl ToString for PolicyAssociationReleaseCause {
		fn to_string(&self) -> String {
			match *self {
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UeSubscription => "UE_SUBSCRIPTION".to_string(),
				Self::InsufficientRes => "INSUFFICIENT_RES".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PolicyAssociationReleaseCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNSPECIFIED" => Ok(Self::Unspecified),
				"UE_SUBSCRIPTION" => Ok(Self::UeSubscription),
				"INSUFFICIENT_RES" => Ok(Self::InsufficientRes),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - UNSPECIFIED: This value is used for unspecified reasons.
	/// - UE_SUBSCRIPTION: This value is used to indicate that the policy
	///   association needs to be
	///  terminated because the subscription of UE has changed (e.g. was
	/// removed).
	/// - INSUFFICIENT_RES: This value is used to indicate that the server is
	///   overloaded and needs
	///  to abort the policy association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNSPECIFIED: This value is used
	/// for unspecified reasons.\n- UE_SUBSCRIPTION: This value is used to
	/// indicate that the policy association needs to be\n  terminated because
	/// the subscription of UE has changed (e.g. was removed).\n-
	/// INSUFFICIENT_RES: This value is used to indicate that the server is
	/// overloaded and needs\n  to abort the policy association.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNSPECIFIED",
	///    "UE_SUBSCRIPTION",
	///    "INSUFFICIENT_RES"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PolicyAssociationReleaseCause1 {
		#[default]
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(rename = "UE_SUBSCRIPTION")]
		UeSubscription,
		#[serde(rename = "INSUFFICIENT_RES")]
		InsufficientRes,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PolicyAssociationReleaseCause1> for PolicyAssociationReleaseCause1 {
		fn from(value: &PolicyAssociationReleaseCause1) -> Self {
			value.clone()
		}
	}

	impl ToString for PolicyAssociationReleaseCause1 {
		fn to_string(&self) -> String {
			match *self {
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UeSubscription => "UE_SUBSCRIPTION".to_string(),
				Self::InsufficientRes => "INSUFFICIENT_RES".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PolicyAssociationReleaseCause1 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNSPECIFIED" => Ok(Self::Unspecified),
				"UE_SUBSCRIPTION" => Ok(Self::UeSubscription),
				"INSUFFICIENT_RES" => Ok(Self::InsufficientRes),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PolicyAssociationReleaseCause1 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PolicyAssociationReleaseCause1 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PolicyAssociationReleaseCause1 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information which the NF service consumer provides when requesting the
	/// creation of a policy association. The serviveName property corresponds
	/// to the serviceName in the main body of the specification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information which the NF service consumer provides when
	/// requesting the creation of a policy association. The serviveName
	/// property corresponds to the serviceName in the main body of the
	/// specification.\n",
	///  "type": "object",
	///  "required": [
	///    "notificationUri",
	///    "supi",
	///    "suppFeat"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "accessTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the 3GPP access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
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
	///      "description": "Alternate or backup IPv4 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv6Addrs": {
	///      "description": "Alternate or backup IPv6 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "mappingSnssais": {
	///      "description": "mapping of each S-NSSAI of the Allowed NSSAI to the
	/// corresponding S-NSSAI of the HPLMN. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MappingOfSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "n3gAllowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the Non-3GPP
	/// access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nwdafDatas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-NwdafData"
	///      },
	///      "minItems": 1
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "ratTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "rfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "servAreaRes": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "servingPlmn": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "serviveName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetSnssais": {
	///      "description": "array of target S-NSSAIs.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "timeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "traceReq": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "ueAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "ueSliceMbrs": {
	///      "description": "The subscribed UE Slice-MBR for each subscribed
	/// S-NSSAI of the home PLMN mapping  to a S-NSSAI of the serving PLMN Shall
	/// be provided when available.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeSliceMbr"
	///      },
	///      "minItems": 1
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
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
	pub struct PolicyAssociationRequest {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(rename = "accessTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub access_types: Vec<AccessType>,
		/// array of allowed S-NSSAIs for the 3GPP access.
		#[serde(
			rename = "allowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_snssais: Vec<Snssai>,
		/// Alternate or backup FQDN(s) where to send Notifications.
		#[serde(
			rename = "altNotifFqdns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_fqdns: Vec<Fqdn>,
		/// Alternate or backup IPv4 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv4Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv4_addrs: Vec<Ipv4Addr>,
		/// Alternate or backup IPv6 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv6Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv6_addrs: Vec<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
		pub group_ids: Vec<GroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		/// mapping of each S-NSSAI of the Allowed NSSAI to the corresponding
		/// S-NSSAI of the HPLMN.
		#[serde(
			rename = "mappingSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mapping_snssais: Vec<MappingOfSnssai>,
		/// array of allowed S-NSSAIs for the Non-3GPP access.
		#[serde(
			rename = "n3gAllowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n3g_allowed_snssais: Vec<Snssai>,
		#[serde(rename = "notificationUri")]
		pub notification_uri: Uri,
		#[serde(rename = "nwdafDatas", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_datas: Vec<SchemasNwdafData>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(rename = "ratTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub rat_types: Vec<RatType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rfsp: Option<RfspIndex>,
		#[serde(
			rename = "servAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_area_res: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "servingPlmn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_plmn: Option<PlmnIdNid>,
		#[serde(
			rename = "serviveName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub servive_name: Option<ServiceName>,
		pub supi: Supi,
		#[serde(rename = "suppFeat")]
		pub supp_feat: SupportedFeatures,
		/// array of target S-NSSAIs.
		#[serde(
			rename = "targetSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub target_snssais: Vec<Snssai>,
		#[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
		pub time_zone: Option<TimeZone>,
		#[serde(rename = "traceReq", default, skip_serializing_if = "Option::is_none")]
		pub trace_req: Option<TraceData>,
		#[serde(rename = "ueAmbr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ambr: Option<Ambr>,
		/// The subscribed UE Slice-MBR for each subscribed S-NSSAI of the home
		/// PLMN mapping  to a S-NSSAI of the serving PLMN Shall be provided
		/// when available.
		#[serde(rename = "ueSliceMbrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_slice_mbrs: Vec<UeSliceMbr>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
		#[serde(
			rename = "wlServAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wl_serv_area_res: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&PolicyAssociationRequest> for PolicyAssociationRequest {
		fn from(value: &PolicyAssociationRequest) -> Self {
			value.clone()
		}
	}

	/// Represents information that the NF service consumer provides when
	/// requesting the creation of a policy association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information that the NF service consumer
	/// provides when requesting the creation of a policy association.\n",
	///  "type": "object",
	///  "required": [
	///    "notificationUri",
	///    "supi",
	///    "suppFeat"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
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
	///      "description": "Alternate or backup IPv4 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv6Addrs": {
	///      "description": "Alternate or backup IPv6 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "hPcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pc5Capab": {
	///      "$ref": "#/components/schemas/Pc5Capability"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "proSeCapab": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ProSeCapability"
	///      },
	///      "minItems": 1
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "serviceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "servingNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "servingPlmn": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "timeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "uePolReq": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PolicyAssociationRequest1 {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		/// Alternate or backup FQDN(s) where to send Notifications.
		#[serde(
			rename = "altNotifFqdns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_fqdns: Vec<Fqdn>,
		/// Alternate or backup IPv4 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv4Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv4_addrs: Vec<Ipv4Addr>,
		/// Alternate or backup IPv6 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv6Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv6_addrs: Vec<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
		pub group_ids: Vec<GroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(rename = "hPcfId", default, skip_serializing_if = "Option::is_none")]
		pub h_pcf_id: Option<NfInstanceId>,
		#[serde(rename = "notificationUri")]
		pub notification_uri: Uri,
		#[serde(rename = "pc5Capab", default, skip_serializing_if = "Option::is_none")]
		pub pc5_capab: Option<Pc5Capability>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "proSeCapab", default, skip_serializing_if = "Vec::is_empty")]
		pub pro_se_capab: Vec<ProSeCapability>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "serviceName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_name: Option<ServiceName>,
		#[serde(
			rename = "servingNfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_nf_id: Option<NfInstanceId>,
		#[serde(
			rename = "servingPlmn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_plmn: Option<PlmnIdNid>,
		pub supi: Supi,
		#[serde(rename = "suppFeat")]
		pub supp_feat: SupportedFeatures,
		#[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
		pub time_zone: Option<TimeZone>,
		#[serde(rename = "uePolReq", default, skip_serializing_if = "Option::is_none")]
		pub ue_pol_req: Option<Bytes>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
	}

	impl From<&PolicyAssociationRequest1> for PolicyAssociationRequest1 {
		fn from(value: &PolicyAssociationRequest1) -> Self {
			value.clone()
		}
	}

	/// Represents information that the NF service consumer provides when
	/// requesting the update of a policy association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information that the NF service consumer
	/// provides when requesting the update of a policy association.\n",
	///  "type": "object",
	///  "properties": {
	///    "accessTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the 3GPP access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
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
	///      "description": "Alternate or backup IPv4 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv6Addrs": {
	///      "description": "Alternate or backup IPv6 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "mappingSnssais": {
	///      "description": "mapping of each S-NSSAI of the Allowed NSSAI to the
	/// corresponding S-NSSAI of the HPLMN. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MappingOfSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "n3gAllowedSnssais": {
	///      "description": "array of allowed S-NSSAIs for the Non-3GPP
	/// access.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nwdafDatas": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-NwdafData"
	///      },
	///      "minItems": 1
	///    },
	///    "praStatuses": {
	///      "description": "Contains the UE presence status for tracking area
	/// for which changes of the UE presence occurred. The praId attribute
	/// within the PresenceInfo data type is the key of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "ratTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "rfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "servAreaRes": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "smfSelInfo": {
	///      "$ref": "#/components/schemas/SmfSelectionData"
	///    },
	///    "targetSnssais": {
	///      "description": "array of target S-NSSAIs.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "traceReq": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the NF service consumer
	/// observes.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "ueAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "ueSliceMbrs": {
	///      "description": "The subscribed UE-Slice-MBR for each subscribed
	/// S-NSSAI of the home PLMN mapping to a S-NSSAI of the serving PLMN Shall
	/// be provided for the \"UE_SLICE_MBR_CH\" policy control request
	/// trigger.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeSliceMbr"
	///      },
	///      "minItems": 1
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
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
	pub struct PolicyAssociationUpdateRequest {
		#[serde(rename = "accessTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub access_types: Vec<AccessType>,
		/// array of allowed S-NSSAIs for the 3GPP access.
		#[serde(
			rename = "allowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_snssais: Vec<Snssai>,
		/// Alternate or backup FQDN(s) where to send Notifications.
		#[serde(
			rename = "altNotifFqdns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_fqdns: Vec<Fqdn>,
		/// Alternate or backup IPv4 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv4Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv4_addrs: Vec<Ipv4Addr>,
		/// Alternate or backup IPv6 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv6Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv6_addrs: Vec<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		/// mapping of each S-NSSAI of the Allowed NSSAI to the corresponding
		/// S-NSSAI of the HPLMN.
		#[serde(
			rename = "mappingSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mapping_snssais: Vec<MappingOfSnssai>,
		/// array of allowed S-NSSAIs for the Non-3GPP access.
		#[serde(
			rename = "n3gAllowedSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n3g_allowed_snssais: Vec<Snssai>,
		#[serde(
			rename = "notificationUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notification_uri: Option<Uri>,
		#[serde(
			rename = "nwdafDatas",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_datas: Option<Vec<SchemasNwdafData>>,
		/// Contains the UE presence status for tracking area for which changes
		/// of the UE presence occurred. The praId attribute within the
		/// PresenceInfo data type is the key of the map.
		#[serde(
			rename = "praStatuses",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_statuses: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "ratTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub rat_types: Vec<RatType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rfsp: Option<RfspIndex>,
		#[serde(
			rename = "servAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_area_res: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "smfSelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_sel_info: Option<SmfSelectionData>,
		/// array of target S-NSSAIs.
		#[serde(
			rename = "targetSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub target_snssais: Vec<Snssai>,
		#[serde(rename = "traceReq", default, skip_serializing_if = "Option::is_none")]
		pub trace_req: Option<TraceData>,
		/// Request Triggers that the NF service consumer observes.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<RequestTrigger>,
		#[serde(rename = "ueAmbr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ambr: Option<Ambr>,
		/// The subscribed UE-Slice-MBR for each subscribed S-NSSAI of the home
		/// PLMN mapping to a S-NSSAI of the serving PLMN Shall be provided for
		/// the "UE_SLICE_MBR_CH" policy control request trigger.
		#[serde(rename = "ueSliceMbrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_slice_mbrs: Vec<UeSliceMbr>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
		#[serde(
			rename = "wlServAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wl_serv_area_res: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&PolicyAssociationUpdateRequest> for PolicyAssociationUpdateRequest {
		fn from(value: &PolicyAssociationUpdateRequest) -> Self {
			value.clone()
		}
	}

	/// Represents Information that the NF service consumer provides when
	/// requesting the update of a policy association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents Information that the NF service consumer
	/// provides when requesting the update of a policy association.\n",
	///  "type": "object",
	///  "properties": {
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
	///      "description": "Alternate or backup IPv4 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv6Addrs": {
	///      "description": "Alternate or backup IPv6 Address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "connectState": {
	///      "$ref": "#/components/schemas/CmState"
	///    },
	///    "groupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "praStatuses": {
	///      "description": "Contains the UE presence status for tracking area
	/// for which changes of the UE presence occurred. The praId attribute
	/// within the PresenceInfo data type is the key of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "proSeCapab": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ProSeCapability"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the NF service consumer
	/// observes.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger1"
	///      },
	///      "minItems": 1
	///    },
	///    "uePolDelResult": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "uePolReq": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "uePolTransFailNotif": {
	///      "$ref": "#/components/schemas/UePolicyTransferFailureNotification"
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PolicyAssociationUpdateRequest1 {
		/// Alternate or backup FQDN(s) where to send Notifications.
		#[serde(
			rename = "altNotifFqdns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_fqdns: Vec<Fqdn>,
		/// Alternate or backup IPv4 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv4Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv4_addrs: Vec<Ipv4Addr>,
		/// Alternate or backup IPv6 Address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv6Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv6_addrs: Vec<Ipv6Addr>,
		#[serde(
			rename = "connectState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub connect_state: Option<CmState>,
		#[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
		pub group_ids: Vec<GroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "notificationUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notification_uri: Option<Uri>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnIdNid>,
		/// Contains the UE presence status for tracking area for which changes
		/// of the UE presence occurred. The praId attribute within the
		/// PresenceInfo data type is the key of the map.
		#[serde(
			rename = "praStatuses",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_statuses: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "proSeCapab", default, skip_serializing_if = "Vec::is_empty")]
		pub pro_se_capab: Vec<ProSeCapability>,
		#[serde(
			rename = "servingNfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_nf_id: Option<NfInstanceId>,
		/// Request Triggers that the NF service consumer observes.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<RequestTrigger1>,
		#[serde(
			rename = "uePolDelResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_pol_del_result: Option<Bytes>,
		#[serde(rename = "uePolReq", default, skip_serializing_if = "Option::is_none")]
		pub ue_pol_req: Option<Bytes>,
		#[serde(
			rename = "uePolTransFailNotif",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_pol_trans_fail_notif: Option<UePolicyTransferFailureNotification>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
	}

	impl From<&PolicyAssociationUpdateRequest1> for PolicyAssociationUpdateRequest1 {
		fn from(value: &PolicyAssociationUpdateRequest1) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - PLMN_CH: PLMN Change
	/// - RES_MO_RE: A request for resource modification has been received by
	///   the SMF. The SMF always reports to the PCF.
	/// - AC_TY_CH: Access Type Change
	/// - UE_IP_CH: UE IP address change. The SMF always reports to the PCF.
	/// - UE_MAC_CH: A new UE MAC address is detected or a used UE MAC address
	///   is inactive for a specific period
	/// - AN_CH_COR: Access Network Charging Correlation Information
	/// - US_RE: The PDU Session or the Monitoring key specific resources
	///   consumed by a UE either reached the threshold or needs to be reported
	///   for other reasons.
	/// - APP_STA: The start of application traffic has been detected.
	/// - APP_STO: The stop of application traffic has been detected.
	/// - AN_INFO: Access Network Information report
	/// - CM_SES_FAIL: Credit management session failure
	/// - PS_DA_OFF: The SMF reports when the 3GPP PS Data Off status changes.
	///   The SMF always reports to the PCF.
	/// - DEF_QOS_CH: Default QoS Change. The SMF always reports to the PCF.
	/// - SE_AMBR_CH: Session-AMBR Change. The SMF always reports to the PCF.
	/// - QOS_NOTIF: The SMF notify the PCF when receiving notification from RAN
	///   that QoS targets of the QoS Flow cannot be guranteed or gurateed
	///   again.
	/// - NO_CREDIT: Out of credit
	/// - REALLO_OF_CREDIT: Reallocation of credit
	/// - PRA_CH: Change of UE presence in Presence Reporting Area
	/// - SAREA_CH: Location Change with respect to the Serving Area
	/// - SCNN_CH: Location Change with respect to the Serving CN node
	/// - RE_TIMEOUT: Indicates the SMF generated the request because there has
	///   been a PCC revalidation timeout
	/// - RES_RELEASE: Indicate that the SMF can inform the PCF of the outcome
	///   of the release of resources for those rules that require so.
	/// - SUCC_RES_ALLO: Indicates that the requested rule data is the
	///   successful resource allocation.
	/// - RAI_CH: Location Change with respect to the RAI of GERAN and UTRAN.
	/// - RAT_TY_CH: RAT Type Change.
	/// - REF_QOS_IND_CH: Reflective QoS indication Change
	/// - NUM_OF_PACKET_FILTER: Indicates that the SMF shall report the number
	///   of supported packet filter for signalled QoS rules
	/// - UE_STATUS_RESUME: Indicates that the UE's status is resumed.
	/// - UE_TZ_CH: UE Time Zone Change
	/// - AUTH_PROF_CH: The DN-AAA authorization profile index has changed
	/// - QOS_MONITORING: Indicate that the SMF notifies the PCF of the QoS
	///   Monitoring information.
	/// - SCELL_CH: Location Change with respect to the Serving Cell.
	/// - USER_LOCATION_CH: Indicate that user location has been changed,
	///   applicable to serving area change and serving cell change.
	/// - EPS_FALLBACK: EPS Fallback report is enabled in the SMF.
	/// - MA_PDU: UE Indicates that the SMF notifies the PCF of the MA PDU
	///   session request
	/// - TSN_BRIDGE_INFO: TSC user plane node information available
	/// - 5G_RG_JOIN: The 5G-RG has joined to an IP Multicast Group.
	/// - 5G_RG_LEAVE: The 5G-RG has left an IP Multicast Group.
	/// - DDN_FAILURE: Event subscription for DDN Failure event received.
	/// - DDN_DELIVERY_STATUS: Event subscription for DDN Delivery Status
	///   received.
	/// - GROUP_ID_LIST_CHG: UE Internal Group Identifier(s) has changed: the
	///   SMF reports that UDM provided list of group Ids has changed.
	/// - DDN_FAILURE_CANCELLATION: The event subscription for DDN Failure event
	///   is cancelled.
	/// - DDN_DELIVERY_STATUS_CANCELLATION: The event subscription for DDD
	///   STATUS is cancelled.
	/// - VPLMN_QOS_CH: Change of the QoS supported in the VPLMN.
	/// - SUCC_QOS_UPDATE: Indicates that the requested MPS Action is
	///   successful.
	/// - SAT_CATEGORY_CHG: Indicates that the SMF has detected a change between
	///   different satellite backhaul categories, or between a satellite
	///   backhaul and a non-satellite backhaul.
	/// - PCF_UE_NOTIF_IND: Indicates the SMF has detected the AMF forwarded the
	///   PCF for the UE indication to receive/stop receiving notifications of
	///   SM Policy association established/terminated events.
	/// - NWDAF_DATA_CHG: Indicates that the NWDAF instance IDs used for the PDU
	///   session and/or associated Analytics IDs used for the PDU session and
	///   available in the SMF have changed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- PLMN_CH: PLMN Change\n-
	/// RES_MO_RE: A request for resource modification has been received by the
	/// SMF. The SMF always reports to the PCF.\n- AC_TY_CH: Access Type
	/// Change\n- UE_IP_CH: UE IP address change. The SMF always reports to the
	/// PCF.\n- UE_MAC_CH: A new UE MAC address is detected or a used UE MAC
	/// address is inactive for a specific period\n- AN_CH_COR: Access Network
	/// Charging Correlation Information\n- US_RE: The PDU Session or the
	/// Monitoring key specific resources consumed by a UE either reached the
	/// threshold or needs to be reported for other reasons.\n- APP_STA: The
	/// start of application traffic has been detected.\n- APP_STO: The stop of
	/// application traffic has been detected.\n- AN_INFO: Access Network
	/// Information report\n- CM_SES_FAIL: Credit management session failure\n-
	/// PS_DA_OFF: The SMF reports when the 3GPP PS Data Off status changes. The
	/// SMF always reports to the PCF.\n- DEF_QOS_CH: Default QoS Change. The
	/// SMF always reports to the PCF.\n- SE_AMBR_CH: Session-AMBR Change. The
	/// SMF always reports to the PCF.\n- QOS_NOTIF: The SMF notify the PCF when
	/// receiving notification from RAN that QoS targets of the QoS Flow cannot
	/// be guranteed or gurateed again.\n- NO_CREDIT: Out of credit\n-
	/// REALLO_OF_CREDIT: Reallocation of credit\n- PRA_CH: Change of UE
	/// presence in Presence Reporting Area\n- SAREA_CH: Location Change with
	/// respect to the Serving Area\n- SCNN_CH: Location Change with respect to
	/// the Serving CN node\n- RE_TIMEOUT: Indicates the SMF generated the
	/// request because there has been a PCC revalidation timeout\n-
	/// RES_RELEASE: Indicate that the SMF can inform the PCF of the outcome of
	/// the release of resources for those rules that require so.\n-
	/// SUCC_RES_ALLO: Indicates that the requested rule data is the successful
	/// resource allocation.\n- RAI_CH: Location Change with respect to the RAI
	/// of GERAN and UTRAN.\n- RAT_TY_CH: RAT Type Change.\n- REF_QOS_IND_CH:
	/// Reflective QoS indication Change\n- NUM_OF_PACKET_FILTER: Indicates that
	/// the SMF shall report the number of supported packet filter for signalled
	/// QoS rules\n- UE_STATUS_RESUME: Indicates that the UE's status is
	/// resumed.\n- UE_TZ_CH: UE Time Zone Change\n- AUTH_PROF_CH: The DN-AAA
	/// authorization profile index has changed\n- QOS_MONITORING: Indicate that
	/// the SMF notifies the PCF of the QoS Monitoring information.\n- SCELL_CH:
	/// Location Change with respect to the Serving Cell.\n- USER_LOCATION_CH:
	/// Indicate that user location has been changed, applicable to serving area
	/// change and serving cell change.\n- EPS_FALLBACK: EPS Fallback report is
	/// enabled in the SMF.\n- MA_PDU: UE Indicates that the SMF notifies the
	/// PCF of the MA PDU session request\n- TSN_BRIDGE_INFO: TSC user plane
	/// node information available\n- 5G_RG_JOIN: The 5G-RG has joined to an IP
	/// Multicast Group.\n- 5G_RG_LEAVE: The 5G-RG has left an IP Multicast
	/// Group.\n- DDN_FAILURE: Event subscription for DDN Failure event
	/// received.\n- DDN_DELIVERY_STATUS: Event subscription for DDN Delivery
	/// Status received.\n- GROUP_ID_LIST_CHG: UE Internal Group Identifier(s)
	/// has changed: the SMF reports that UDM provided list of group Ids has
	/// changed.\n- DDN_FAILURE_CANCELLATION: The event subscription for DDN
	/// Failure event is cancelled.\n- DDN_DELIVERY_STATUS_CANCELLATION: The
	/// event subscription for DDD STATUS is cancelled.\n- VPLMN_QOS_CH: Change
	/// of the QoS supported in the VPLMN.\n- SUCC_QOS_UPDATE: Indicates that
	/// the requested MPS Action is successful.\n- SAT_CATEGORY_CHG: Indicates
	/// that the SMF has detected a change between different satellite backhaul
	/// categories, or between a satellite backhaul and a non-satellite
	/// backhaul.\n- PCF_UE_NOTIF_IND: Indicates the SMF has detected the AMF
	/// forwarded the PCF for the UE indication to receive/stop receiving
	/// notifications of SM Policy association established/terminated events.\n-
	/// NWDAF_DATA_CHG: Indicates that the NWDAF instance IDs used for the PDU
	/// session and/or associated Analytics IDs used for the PDU session and
	/// available in the SMF have changed.\n",
	///  "type": "string",
	///  "enum": [
	///    "PLMN_CH",
	///    "RES_MO_RE",
	///    "AC_TY_CH",
	///    "UE_IP_CH",
	///    "UE_MAC_CH",
	///    "AN_CH_COR",
	///    "US_RE",
	///    "APP_STA",
	///    "APP_STO",
	///    "AN_INFO",
	///    "CM_SES_FAIL",
	///    "PS_DA_OFF",
	///    "DEF_QOS_CH",
	///    "SE_AMBR_CH",
	///    "QOS_NOTIF",
	///    "NO_CREDIT",
	///    "REALLO_OF_CREDIT",
	///    "PRA_CH",
	///    "SAREA_CH",
	///    "SCNN_CH",
	///    "RE_TIMEOUT",
	///    "RES_RELEASE",
	///    "SUCC_RES_ALLO",
	///    "RAI_CH",
	///    "RAT_TY_CH",
	///    "REF_QOS_IND_CH",
	///    "NUM_OF_PACKET_FILTER",
	///    "UE_STATUS_RESUME",
	///    "UE_TZ_CH",
	///    "AUTH_PROF_CH",
	///    "QOS_MONITORING",
	///    "SCELL_CH",
	///    "USER_LOCATION_CH",
	///    "EPS_FALLBACK",
	///    "MA_PDU",
	///    "TSN_BRIDGE_INFO",
	///    "5G_RG_JOIN",
	///    "5G_RG_LEAVE",
	///    "DDN_FAILURE",
	///    "DDN_DELIVERY_STATUS",
	///    "GROUP_ID_LIST_CHG",
	///    "DDN_FAILURE_CANCELLATION",
	///    "DDN_DELIVERY_STATUS_CANCELLATION",
	///    "VPLMN_QOS_CH",
	///    "SUCC_QOS_UPDATE",
	///    "SAT_CATEGORY_CHG",
	///    "PCF_UE_NOTIF_IND",
	///    "NWDAF_DATA_CHG"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PolicyControlRequestTrigger {
		#[default]
		#[serde(rename = "PLMN_CH")]
		PlmnCh,
		#[serde(rename = "RES_MO_RE")]
		ResMoRe,
		#[serde(rename = "AC_TY_CH")]
		AcTyCh,
		#[serde(rename = "UE_IP_CH")]
		UeIpCh,
		#[serde(rename = "UE_MAC_CH")]
		UeMacCh,
		#[serde(rename = "AN_CH_COR")]
		AnChCor,
		#[serde(rename = "US_RE")]
		UsRe,
		#[serde(rename = "APP_STA")]
		AppSta,
		#[serde(rename = "APP_STO")]
		AppSto,
		#[serde(rename = "AN_INFO")]
		AnInfo,
		#[serde(rename = "CM_SES_FAIL")]
		CmSesFail,
		#[serde(rename = "PS_DA_OFF")]
		PsDaOff,
		#[serde(rename = "DEF_QOS_CH")]
		DefQosCh,
		#[serde(rename = "SE_AMBR_CH")]
		SeAmbrCh,
		#[serde(rename = "QOS_NOTIF")]
		QosNotif,
		#[serde(rename = "NO_CREDIT")]
		NoCredit,
		#[serde(rename = "REALLO_OF_CREDIT")]
		RealloOfCredit,
		#[serde(rename = "PRA_CH")]
		PraCh,
		#[serde(rename = "SAREA_CH")]
		SareaCh,
		#[serde(rename = "SCNN_CH")]
		ScnnCh,
		#[serde(rename = "RE_TIMEOUT")]
		ReTimeout,
		#[serde(rename = "RES_RELEASE")]
		ResRelease,
		#[serde(rename = "SUCC_RES_ALLO")]
		SuccResAllo,
		#[serde(rename = "RAI_CH")]
		RaiCh,
		#[serde(rename = "RAT_TY_CH")]
		RatTyCh,
		#[serde(rename = "REF_QOS_IND_CH")]
		RefQosIndCh,
		#[serde(rename = "NUM_OF_PACKET_FILTER")]
		NumOfPacketFilter,
		#[serde(rename = "UE_STATUS_RESUME")]
		UeStatusResume,
		#[serde(rename = "UE_TZ_CH")]
		UeTzCh,
		#[serde(rename = "AUTH_PROF_CH")]
		AuthProfCh,
		#[serde(rename = "QOS_MONITORING")]
		QosMonitoring,
		#[serde(rename = "SCELL_CH")]
		ScellCh,
		#[serde(rename = "USER_LOCATION_CH")]
		UserLocationCh,
		#[serde(rename = "EPS_FALLBACK")]
		EpsFallback,
		#[serde(rename = "MA_PDU")]
		MaPdu,
		#[serde(rename = "TSN_BRIDGE_INFO")]
		TsnBridgeInfo,
		#[serde(rename = "5G_RG_JOIN")]
		FiveGRgJoin,
		#[serde(rename = "5G_RG_LEAVE")]
		FiveGRgLeave,
		#[serde(rename = "DDN_FAILURE")]
		DdnFailure,
		#[serde(rename = "DDN_DELIVERY_STATUS")]
		DdnDeliveryStatus,
		#[serde(rename = "GROUP_ID_LIST_CHG")]
		GroupIdListChg,
		#[serde(rename = "DDN_FAILURE_CANCELLATION")]
		DdnFailureCancellation,
		#[serde(rename = "DDN_DELIVERY_STATUS_CANCELLATION")]
		DdnDeliveryStatusCancellation,
		#[serde(rename = "VPLMN_QOS_CH")]
		VplmnQosCh,
		#[serde(rename = "SUCC_QOS_UPDATE")]
		SuccQosUpdate,
		#[serde(rename = "SAT_CATEGORY_CHG")]
		SatCategoryChg,
		#[serde(rename = "PCF_UE_NOTIF_IND")]
		PcfUeNotifInd,
		#[serde(rename = "NWDAF_DATA_CHG")]
		NwdafDataChg,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PolicyControlRequestTrigger> for PolicyControlRequestTrigger {
		fn from(value: &PolicyControlRequestTrigger) -> Self {
			value.clone()
		}
	}

	impl ToString for PolicyControlRequestTrigger {
		fn to_string(&self) -> String {
			match *self {
				Self::PlmnCh => "PLMN_CH".to_string(),
				Self::ResMoRe => "RES_MO_RE".to_string(),
				Self::AcTyCh => "AC_TY_CH".to_string(),
				Self::UeIpCh => "UE_IP_CH".to_string(),
				Self::UeMacCh => "UE_MAC_CH".to_string(),
				Self::AnChCor => "AN_CH_COR".to_string(),
				Self::UsRe => "US_RE".to_string(),
				Self::AppSta => "APP_STA".to_string(),
				Self::AppSto => "APP_STO".to_string(),
				Self::AnInfo => "AN_INFO".to_string(),
				Self::CmSesFail => "CM_SES_FAIL".to_string(),
				Self::PsDaOff => "PS_DA_OFF".to_string(),
				Self::DefQosCh => "DEF_QOS_CH".to_string(),
				Self::SeAmbrCh => "SE_AMBR_CH".to_string(),
				Self::QosNotif => "QOS_NOTIF".to_string(),
				Self::NoCredit => "NO_CREDIT".to_string(),
				Self::RealloOfCredit => "REALLO_OF_CREDIT".to_string(),
				Self::PraCh => "PRA_CH".to_string(),
				Self::SareaCh => "SAREA_CH".to_string(),
				Self::ScnnCh => "SCNN_CH".to_string(),
				Self::ReTimeout => "RE_TIMEOUT".to_string(),
				Self::ResRelease => "RES_RELEASE".to_string(),
				Self::SuccResAllo => "SUCC_RES_ALLO".to_string(),
				Self::RaiCh => "RAI_CH".to_string(),
				Self::RatTyCh => "RAT_TY_CH".to_string(),
				Self::RefQosIndCh => "REF_QOS_IND_CH".to_string(),
				Self::NumOfPacketFilter => "NUM_OF_PACKET_FILTER".to_string(),
				Self::UeStatusResume => "UE_STATUS_RESUME".to_string(),
				Self::UeTzCh => "UE_TZ_CH".to_string(),
				Self::AuthProfCh => "AUTH_PROF_CH".to_string(),
				Self::QosMonitoring => "QOS_MONITORING".to_string(),
				Self::ScellCh => "SCELL_CH".to_string(),
				Self::UserLocationCh => "USER_LOCATION_CH".to_string(),
				Self::EpsFallback => "EPS_FALLBACK".to_string(),
				Self::MaPdu => "MA_PDU".to_string(),
				Self::TsnBridgeInfo => "TSN_BRIDGE_INFO".to_string(),
				Self::FiveGRgJoin => "5G_RG_JOIN".to_string(),
				Self::FiveGRgLeave => "5G_RG_LEAVE".to_string(),
				Self::DdnFailure => "DDN_FAILURE".to_string(),
				Self::DdnDeliveryStatus => "DDN_DELIVERY_STATUS".to_string(),
				Self::GroupIdListChg => "GROUP_ID_LIST_CHG".to_string(),
				Self::DdnFailureCancellation => "DDN_FAILURE_CANCELLATION".to_string(),
				Self::DdnDeliveryStatusCancellation => {
					"DDN_DELIVERY_STATUS_CANCELLATION".to_string()
				}
				Self::VplmnQosCh => "VPLMN_QOS_CH".to_string(),
				Self::SuccQosUpdate => "SUCC_QOS_UPDATE".to_string(),
				Self::SatCategoryChg => "SAT_CATEGORY_CHG".to_string(),
				Self::PcfUeNotifInd => "PCF_UE_NOTIF_IND".to_string(),
				Self::NwdafDataChg => "NWDAF_DATA_CHG".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PolicyControlRequestTrigger {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PLMN_CH" => Ok(Self::PlmnCh),
				"RES_MO_RE" => Ok(Self::ResMoRe),
				"AC_TY_CH" => Ok(Self::AcTyCh),
				"UE_IP_CH" => Ok(Self::UeIpCh),
				"UE_MAC_CH" => Ok(Self::UeMacCh),
				"AN_CH_COR" => Ok(Self::AnChCor),
				"US_RE" => Ok(Self::UsRe),
				"APP_STA" => Ok(Self::AppSta),
				"APP_STO" => Ok(Self::AppSto),
				"AN_INFO" => Ok(Self::AnInfo),
				"CM_SES_FAIL" => Ok(Self::CmSesFail),
				"PS_DA_OFF" => Ok(Self::PsDaOff),
				"DEF_QOS_CH" => Ok(Self::DefQosCh),
				"SE_AMBR_CH" => Ok(Self::SeAmbrCh),
				"QOS_NOTIF" => Ok(Self::QosNotif),
				"NO_CREDIT" => Ok(Self::NoCredit),
				"REALLO_OF_CREDIT" => Ok(Self::RealloOfCredit),
				"PRA_CH" => Ok(Self::PraCh),
				"SAREA_CH" => Ok(Self::SareaCh),
				"SCNN_CH" => Ok(Self::ScnnCh),
				"RE_TIMEOUT" => Ok(Self::ReTimeout),
				"RES_RELEASE" => Ok(Self::ResRelease),
				"SUCC_RES_ALLO" => Ok(Self::SuccResAllo),
				"RAI_CH" => Ok(Self::RaiCh),
				"RAT_TY_CH" => Ok(Self::RatTyCh),
				"REF_QOS_IND_CH" => Ok(Self::RefQosIndCh),
				"NUM_OF_PACKET_FILTER" => Ok(Self::NumOfPacketFilter),
				"UE_STATUS_RESUME" => Ok(Self::UeStatusResume),
				"UE_TZ_CH" => Ok(Self::UeTzCh),
				"AUTH_PROF_CH" => Ok(Self::AuthProfCh),
				"QOS_MONITORING" => Ok(Self::QosMonitoring),
				"SCELL_CH" => Ok(Self::ScellCh),
				"USER_LOCATION_CH" => Ok(Self::UserLocationCh),
				"EPS_FALLBACK" => Ok(Self::EpsFallback),
				"MA_PDU" => Ok(Self::MaPdu),
				"TSN_BRIDGE_INFO" => Ok(Self::TsnBridgeInfo),
				"5G_RG_JOIN" => Ok(Self::FiveGRgJoin),
				"5G_RG_LEAVE" => Ok(Self::FiveGRgLeave),
				"DDN_FAILURE" => Ok(Self::DdnFailure),
				"DDN_DELIVERY_STATUS" => Ok(Self::DdnDeliveryStatus),
				"GROUP_ID_LIST_CHG" => Ok(Self::GroupIdListChg),
				"DDN_FAILURE_CANCELLATION" => Ok(Self::DdnFailureCancellation),
				"DDN_DELIVERY_STATUS_CANCELLATION" => Ok(Self::DdnDeliveryStatusCancellation),
				"VPLMN_QOS_CH" => Ok(Self::VplmnQosCh),
				"SUCC_QOS_UPDATE" => Ok(Self::SuccQosUpdate),
				"SAT_CATEGORY_CHG" => Ok(Self::SatCategoryChg),
				"PCF_UE_NOTIF_IND" => Ok(Self::PcfUeNotifInd),
				"NWDAF_DATA_CHG" => Ok(Self::NwdafDataChg),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PolicyControlRequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PolicyControlRequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PolicyControlRequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the type of the failed policy decision and/or condition data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the type of the failed policy decision and/or
	/// condition data.",
	///  "type": "string",
	///  "enum": [
	///    "TRA_CTRL_DECS_ERR",
	///    "QOS_DECS_ERR",
	///    "CHG_DECS_ERR",
	///    "USA_MON_DECS_ERR",
	///    "QOS_MON_DECS_ERR",
	///    "CON_DATA_ERR",
	///    "POLICY_PARAM_ERR"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PolicyDecisionFailureCode {
		#[default]
		#[serde(rename = "TRA_CTRL_DECS_ERR")]
		TraCtrlDecsErr,
		#[serde(rename = "QOS_DECS_ERR")]
		QosDecsErr,
		#[serde(rename = "CHG_DECS_ERR")]
		ChgDecsErr,
		#[serde(rename = "USA_MON_DECS_ERR")]
		UsaMonDecsErr,
		#[serde(rename = "QOS_MON_DECS_ERR")]
		QosMonDecsErr,
		#[serde(rename = "CON_DATA_ERR")]
		ConDataErr,
		#[serde(rename = "POLICY_PARAM_ERR")]
		PolicyParamErr,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PolicyDecisionFailureCode> for PolicyDecisionFailureCode {
		fn from(value: &PolicyDecisionFailureCode) -> Self {
			value.clone()
		}
	}

	impl ToString for PolicyDecisionFailureCode {
		fn to_string(&self) -> String {
			match *self {
				Self::TraCtrlDecsErr => "TRA_CTRL_DECS_ERR".to_string(),
				Self::QosDecsErr => "QOS_DECS_ERR".to_string(),
				Self::ChgDecsErr => "CHG_DECS_ERR".to_string(),
				Self::UsaMonDecsErr => "USA_MON_DECS_ERR".to_string(),
				Self::QosMonDecsErr => "QOS_MON_DECS_ERR".to_string(),
				Self::ConDataErr => "CON_DATA_ERR".to_string(),
				Self::PolicyParamErr => "POLICY_PARAM_ERR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PolicyDecisionFailureCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TRA_CTRL_DECS_ERR" => Ok(Self::TraCtrlDecsErr),
				"QOS_DECS_ERR" => Ok(Self::QosDecsErr),
				"CHG_DECS_ERR" => Ok(Self::ChgDecsErr),
				"USA_MON_DECS_ERR" => Ok(Self::UsaMonDecsErr),
				"QOS_MON_DECS_ERR" => Ok(Self::QosMonDecsErr),
				"CON_DATA_ERR" => Ok(Self::ConDataErr),
				"POLICY_PARAM_ERR" => Ok(Self::PolicyParamErr),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PolicyDecisionFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PolicyDecisionFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PolicyDecisionFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents updated policies that the PCF provides in a notification or
	/// in a reply to an Update Request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents updated policies that the PCF provides in a
	/// notification or in a reply to an Update Request.\n",
	///  "type": "object",
	///  "required": [
	///    "resourceUri"
	///  ],
	///  "properties": {
	///    "asTimeDisParam": {
	///      "$ref": "#/components/schemas/AsTimeDistributionParam"
	///    },
	///    "matchPdus": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionInfo"
	///      }
	///    },
	///    "pcfUeInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pras": {
	///      "description": "Contains the presence reporting area(s) for which
	/// reporting was requested. The praId attribute within the PresenceInfo
	/// data type is the key of the map.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfoRm"
	///      }
	///    },
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "rfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "servAreaRes": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "smfSelInfo": {
	///      "$ref": "#/components/schemas/SmfSelectionData"
	///    },
	///    "targetRfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the PCF subscribes.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "ueAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "ueSliceMbrs": {
	///      "description": "One or more UE-Slice-MBR(s) for S-NSSAI(s) of
	/// serving PLMN the allowed NSSAI as part of the AMF Access and Mobility
	/// Policy as determined by the PCF.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeSliceMbr"
	///      },
	///      "minItems": 1
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
	pub struct PolicyUpdate {
		#[serde(
			rename = "asTimeDisParam",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub as_time_dis_param: Option<AsTimeDistributionParam>,
		#[serde(rename = "matchPdus", default, skip_serializing_if = "Option::is_none")]
		pub match_pdus: Option<Vec<PduSessionInfo>>,
		#[serde(rename = "pcfUeInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_ue_info: Option<PcfUeCallbackInfo>,
		/// Contains the presence reporting area(s) for which reporting was
		/// requested. The praId attribute within the PresenceInfo data type is
		/// the key of the map.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pras: Option<::std::collections::HashMap<String, PresenceInfoRm>>,
		#[serde(rename = "resourceUri")]
		pub resource_uri: Uri,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rfsp: Option<RfspIndex>,
		#[serde(
			rename = "servAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_area_res: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "smfSelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_sel_info: Option<SmfSelectionData>,
		#[serde(
			rename = "targetRfsp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_rfsp: Option<RfspIndex>,
		/// Request Triggers that the PCF subscribes.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub triggers: Option<Vec<RequestTrigger>>,
		#[serde(rename = "ueAmbr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ambr: Option<Ambr>,
		/// One or more UE-Slice-MBR(s) for S-NSSAI(s) of serving PLMN the
		/// allowed NSSAI as part of the AMF Access and Mobility Policy as
		/// determined by the PCF.
		#[serde(rename = "ueSliceMbrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_slice_mbrs: Vec<UeSliceMbr>,
		#[serde(
			rename = "wlServAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wl_serv_area_res: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&PolicyUpdate> for PolicyUpdate {
		fn from(value: &PolicyUpdate) -> Self {
			value.clone()
		}
	}

	/// Represents updated policies that the PCF provides in a notification or
	/// in the reply to an Update Request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents updated policies that the PCF provides in a
	/// notification or in the reply to an Update Request.\n",
	///  "type": "object",
	///  "required": [
	///    "resourceUri"
	///  ],
	///  "properties": {
	///    "n2Pc5Pol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "n2Pc5ProSePol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "pras": {
	///      "description": "Contains the presence reporting area(s) for which
	/// reporting was requested. The praId attribute within the PresenceInfo
	/// data type is the key of the map.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "triggers": {
	///      "description": "Request Triggers that the PCF subscribes. Only
	/// values \"LOC_CH\" and \"PRA_CH\" are permitted.\n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RequestTrigger1"
	///      },
	///      "minItems": 1
	///    },
	///    "uePolicy": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PolicyUpdate1 {
		#[serde(rename = "n2Pc5Pol", default, skip_serializing_if = "Option::is_none")]
		pub n2_pc5_pol: Option<N2InfoContent>,
		#[serde(
			rename = "n2Pc5ProSePol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_pc5_pro_se_pol: Option<N2InfoContent>,
		/// Contains the presence reporting area(s) for which reporting was
		/// requested. The praId attribute within the PresenceInfo data type is
		/// the key of the map.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pras: Option<::std::collections::HashMap<String, PresenceInfo>>,
		#[serde(rename = "resourceUri")]
		pub resource_uri: Uri,
		/// Request Triggers that the PCF subscribes. Only values "LOC_CH" and
		/// "PRA_CH" are permitted.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub triggers: Option<Vec<RequestTrigger1>>,
		#[serde(rename = "uePolicy", default, skip_serializing_if = "Option::is_none")]
		pub ue_policy: Option<Bytes>,
	}

	impl From<&PolicyUpdate1> for PolicyUpdate1 {
		fn from(value: &PolicyUpdate1) -> Self {
			value.clone()
		}
	}

	/// Contains the port management information container for a port.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the port management information container for
	/// a port.",
	///  "type": "object",
	///  "required": [
	///    "portManCont",
	///    "portNum"
	///  ],
	///  "properties": {
	///    "portManCont": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "portNum": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PortManagementContainer {
		#[serde(rename = "portManCont")]
		pub port_man_cont: Bytes,
		#[serde(rename = "portNum")]
		pub port_num: Uinteger,
	}

	impl From<&PortManagementContainer> for PortManagementContainer {
		fn from(value: &PortManagementContainer) -> Self {
			value.clone()
		}
	}

	/// Represents Pre-emption control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents Pre-emption control information.",
	///  "type": "string",
	///  "enum": [
	///    "MOST_RECENT",
	///    "LEAST_RECENT",
	///    "HIGHEST_BW"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PreemptionControlInformation {
		#[default]
		#[serde(rename = "MOST_RECENT")]
		MostRecent,
		#[serde(rename = "LEAST_RECENT")]
		LeastRecent,
		#[serde(rename = "HIGHEST_BW")]
		HighestBw,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PreemptionControlInformation> for PreemptionControlInformation {
		fn from(value: &PreemptionControlInformation) -> Self {
			value.clone()
		}
	}

	impl ToString for PreemptionControlInformation {
		fn to_string(&self) -> String {
			match *self {
				Self::MostRecent => "MOST_RECENT".to_string(),
				Self::LeastRecent => "LEAST_RECENT".to_string(),
				Self::HighestBw => "HIGHEST_BW".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PreemptionControlInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MOST_RECENT" => Ok(Self::MostRecent),
				"LEAST_RECENT" => Ok(Self::LeastRecent),
				"HIGHEST_BW" => Ok(Self::HighestBw),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PreemptionControlInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PreemptionControlInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PreemptionControlInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// This data type is defined in the same way as the
	/// PreemptionControlInformation data type, but with the OpenAPI nullable
	/// property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// PreemptionControlInformation data type, but with the OpenAPI nullable
	/// property set to true.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PreemptionControlInformation"
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
	pub enum PreemptionControlInformationRm {
		#[default]
		PreemptionControlInformation(PreemptionControlInformation),
		NullValue(NullValue),
	}

	impl From<&PreemptionControlInformationRm> for PreemptionControlInformationRm {
		fn from(value: &PreemptionControlInformationRm) -> Self {
			value.clone()
		}
	}

	impl From<PreemptionControlInformation> for PreemptionControlInformationRm {
		fn from(value: PreemptionControlInformation) -> Self {
			Self::PreemptionControlInformation(value)
		}
	}

	impl From<NullValue> for PreemptionControlInformationRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Represents the Priority sharing indicator.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Priority sharing indicator.",
	///  "type": "string",
	///  "enum": [
	///    "ENABLED",
	///    "DISABLED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PrioritySharingIndicator {
		#[default]
		#[serde(rename = "ENABLED")]
		Enabled,
		#[serde(rename = "DISABLED")]
		Disabled,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PrioritySharingIndicator> for PrioritySharingIndicator {
		fn from(value: &PrioritySharingIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for PrioritySharingIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::Enabled => "ENABLED".to_string(),
				Self::Disabled => "DISABLED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PrioritySharingIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ENABLED" => Ok(Self::Enabled),
				"DISABLED" => Ok(Self::Disabled),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PrioritySharingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PrioritySharingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PrioritySharingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - PROSE_DD: This value is used to indicate that 5G ProSe Direct
	///   Discovery is supported
	///  by the UE.
	/// - PROSE_DC: This value is used to indicate that 5G ProSe Direct
	///   Communication is supported
	///  by the UE.
	/// - PROSE_L2_U2N_RELAY: This value is used to indicate that Layer-2 5G
	///   ProSe UE-to-Network
	///  Relay is supported by the UE.
	/// - PROSE_L3_U2N_RELAY: This value is used to indicate that Layer-3 5G
	///   ProSe UE-to-Network
	///  Relay is supported by the UE.
	/// - PROSE_L2_REMOTE_UE: This value is used to indicate that Layer-2 5G
	///   ProSe Remote UE is
	///  supported by the UE.
	/// - PROSE_L3_REMOTE_UE: This value is used to indicate that Layer-3 5G
	///   ProSe Remote UE is
	///  supported by the UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- PROSE_DD: This value is used to
	/// indicate that 5G ProSe Direct Discovery is supported\n  by the UE.\n-
	/// PROSE_DC: This value is used to indicate that 5G ProSe Direct
	/// Communication is supported\n  by the UE.\n- PROSE_L2_U2N_RELAY: This
	/// value is used to indicate that Layer-2 5G ProSe UE-to-Network\n  Relay
	/// is supported by the UE.\n- PROSE_L3_U2N_RELAY: This value is used to
	/// indicate that Layer-3 5G ProSe UE-to-Network\n  Relay is supported by
	/// the UE.\n- PROSE_L2_REMOTE_UE: This value is used to indicate that
	/// Layer-2 5G ProSe Remote UE is\n  supported by the UE.\n-
	/// PROSE_L3_REMOTE_UE: This value is used to indicate that Layer-3 5G ProSe
	/// Remote UE is\n  supported by the UE.\n",
	///  "type": "string",
	///  "enum": [
	///    "PROSE_DD",
	///    "PROSE_DC",
	///    "PROSE_L2_U2N_RELAY",
	///    "PROSE_L3_U2N_RELAY",
	///    "PROSE_L2_REMOTE_UE",
	///    "PROSE_L3_REMOTE_UE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ProSeCapability {
		#[default]
		#[serde(rename = "PROSE_DD")]
		ProseDd,
		#[serde(rename = "PROSE_DC")]
		ProseDc,
		#[serde(rename = "PROSE_L2_U2N_RELAY")]
		ProseL2U2nRelay,
		#[serde(rename = "PROSE_L3_U2N_RELAY")]
		ProseL3U2nRelay,
		#[serde(rename = "PROSE_L2_REMOTE_UE")]
		ProseL2RemoteUe,
		#[serde(rename = "PROSE_L3_REMOTE_UE")]
		ProseL3RemoteUe,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ProSeCapability> for ProSeCapability {
		fn from(value: &ProSeCapability) -> Self {
			value.clone()
		}
	}

	impl ToString for ProSeCapability {
		fn to_string(&self) -> String {
			match *self {
				Self::ProseDd => "PROSE_DD".to_string(),
				Self::ProseDc => "PROSE_DC".to_string(),
				Self::ProseL2U2nRelay => "PROSE_L2_U2N_RELAY".to_string(),
				Self::ProseL3U2nRelay => "PROSE_L3_U2N_RELAY".to_string(),
				Self::ProseL2RemoteUe => "PROSE_L2_REMOTE_UE".to_string(),
				Self::ProseL3RemoteUe => "PROSE_L3_REMOTE_UE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ProSeCapability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PROSE_DD" => Ok(Self::ProseDd),
				"PROSE_DC" => Ok(Self::ProseDc),
				"PROSE_L2_U2N_RELAY" => Ok(Self::ProseL2U2nRelay),
				"PROSE_L3_U2N_RELAY" => Ok(Self::ProseL3U2nRelay),
				"PROSE_L2_REMOTE_UE" => Ok(Self::ProseL2RemoteUe),
				"PROSE_L3_REMOTE_UE" => Ok(Self::ProseL3RemoteUe),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ProSeCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProSeCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProSeCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Possible values are
	/// - GENERAL: Indicate no specific QoS flow usage information is available.
	/// - IMS_SIG: Indicate that the QoS flow is used for IMS signalling only.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- GENERAL: Indicate no specific
	/// QoS flow usage information is available. \n- IMS_SIG: Indicate that the
	/// QoS flow is used for IMS signalling only.\n",
	///  "type": "string",
	///  "enum": [
	///    "GENERAL",
	///    "IMS_SIG"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum QosFlowUsage {
		#[default]
		#[serde(rename = "GENERAL")]
		General,
		#[serde(rename = "IMS_SIG")]
		ImsSig,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QosFlowUsage> for QosFlowUsage {
		fn from(value: &QosFlowUsage) -> Self {
			value.clone()
		}
	}

	impl ToString for QosFlowUsage {
		fn to_string(&self) -> String {
			match *self {
				Self::General => "GENERAL".to_string(),
				Self::ImsSig => "IMS_SIG".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QosFlowUsage {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GENERAL" => Ok(Self::General),
				"IMS_SIG" => Ok(Self::ImsSig),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QosFlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QosFlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QosFlowUsage {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains QoS monitoring related control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains QoS monitoring related control information.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "qmId",
	///    "repFreqs",
	///    "reqQosMonParams"
	///  ],
	///  "properties": {
	///    "directNotifInd": {
	///      "description": "Indicates that the direct event notification sent
	/// by UPF to the Local NEF or AF is requested if it is included and set to
	/// true.",
	///      "type": "boolean"
	///    },
	///    "notifyCorreId": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "notifyUri": {
	///      "$ref": "#/components/schemas/UriRm"
	///    },
	///    "qmId": {
	///      "description": "Univocally identifies the QoS monitoring policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "repFreqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReportingFrequency"
	///      },
	///      "minItems": 1
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "repThreshDl": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for DL packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "repThreshRp": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for round trip packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "repThreshUl": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for UL packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "reqQosMonParams": {
	///      "description": "indicates the UL packet delay, DL packet delay
	/// and/or round trip packet delay between the UE and the UPF is to be
	/// monitored when the QoS Monitoring for URLLC is enabled for the service
	/// data flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedQosMonitoringParameter"
	///      },
	///      "minItems": 1
	///    },
	///    "waitTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringData(pub Option<QosMonitoringDataInner>);

	impl ::std::ops::Deref for QosMonitoringData {
		type Target = Option<QosMonitoringDataInner>;
		fn deref(&self) -> &Option<QosMonitoringDataInner> {
			&self.0
		}
	}

	impl From<QosMonitoringData> for Option<QosMonitoringDataInner> {
		fn from(value: QosMonitoringData) -> Self {
			value.0
		}
	}

	impl From<&QosMonitoringData> for QosMonitoringData {
		fn from(value: &QosMonitoringData) -> Self {
			value.clone()
		}
	}

	impl From<Option<QosMonitoringDataInner>> for QosMonitoringData {
		fn from(value: Option<QosMonitoringDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains QoS monitoring related control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains QoS monitoring related control information.",
	///  "type": "object",
	///  "required": [
	///    "qmId",
	///    "repFreqs",
	///    "reqQosMonParams"
	///  ],
	///  "properties": {
	///    "directNotifInd": {
	///      "description": "Indicates that the direct event notification sent
	/// by UPF to the Local NEF or AF is requested if it is included and set to
	/// true.",
	///      "type": "boolean"
	///    },
	///    "notifyCorreId": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "notifyUri": {
	///      "$ref": "#/components/schemas/UriRm"
	///    },
	///    "qmId": {
	///      "description": "Univocally identifies the QoS monitoring policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "repFreqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReportingFrequency"
	///      },
	///      "minItems": 1
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "repThreshDl": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for DL packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "repThreshRp": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for round trip packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "repThreshUl": {
	///      "description": "Indicates the period of time in units of
	/// miliiseconds for UL packet delay.",
	///      "type": [
	///        "integer",
	///        "null"
	///      ]
	///    },
	///    "reqQosMonParams": {
	///      "description": "indicates the UL packet delay, DL packet delay
	/// and/or round trip packet delay between the UE and the UPF is to be
	/// monitored when the QoS Monitoring for URLLC is enabled for the service
	/// data flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedQosMonitoringParameter"
	///      },
	///      "minItems": 1
	///    },
	///    "waitTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringDataInner {
		/// Indicates that the direct event notification sent by UPF to the
		/// Local NEF or AF is requested if it is included and set to true.
		#[serde(
			rename = "directNotifInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_notif_ind: Option<bool>,
		#[serde(
			rename = "notifyCorreId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_corre_id: Option<String>,
		#[serde(rename = "notifyUri", default, skip_serializing_if = "Option::is_none")]
		pub notify_uri: Option<UriRm>,
		/// Univocally identifies the QoS monitoring policy data within a PDU
		/// session.
		#[serde(rename = "qmId")]
		pub qm_id: String,
		#[serde(rename = "repFreqs")]
		pub rep_freqs: Vec<ReportingFrequency>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSecRm>,
		/// Indicates the period of time in units of miliiseconds for DL packet
		/// delay.
		#[serde(
			rename = "repThreshDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_dl: Option<i64>,
		/// Indicates the period of time in units of miliiseconds for round trip
		/// packet delay.
		#[serde(
			rename = "repThreshRp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_rp: Option<i64>,
		/// Indicates the period of time in units of miliiseconds for UL packet
		/// delay.
		#[serde(
			rename = "repThreshUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_ul: Option<i64>,
		/// indicates the UL packet delay, DL packet delay and/or round trip
		/// packet delay between the UE and the UPF is to be monitored when the
		/// QoS Monitoring for URLLC is enabled for the service data flow.
		#[serde(rename = "reqQosMonParams")]
		pub req_qos_mon_params: Vec<RequestedQosMonitoringParameter>,
		#[serde(rename = "waitTime", default, skip_serializing_if = "Option::is_none")]
		pub wait_time: Option<DurationSecRm>,
	}

	impl From<&QosMonitoringDataInner> for QosMonitoringDataInner {
		fn from(value: &QosMonitoringDataInner) -> Self {
			value.clone()
		}
	}

	/// Indicates the QoS Monitoring information to report, i.e. UL and/or DL
	/// and or round trip delay.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the QoS Monitoring information to report,
	/// i.e. UL and/or DL and or round trip delay.",
	///  "type": "object",
	///  "properties": {
	///    "repThreshDl": {
	///      "type": "integer"
	///    },
	///    "repThreshRp": {
	///      "type": "integer"
	///    },
	///    "repThreshUl": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringInformation {
		#[serde(
			rename = "repThreshDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_dl: Option<i64>,
		#[serde(
			rename = "repThreshRp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_rp: Option<i64>,
		#[serde(
			rename = "repThreshUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_ul: Option<i64>,
	}

	impl From<&QosMonitoringInformation> for QosMonitoringInformation {
		fn from(value: &QosMonitoringInformation) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the
	/// QosMonitoringInformation data type, but with the OpenAPI nullable
	/// property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// QosMonitoringInformation data type, but with the OpenAPI nullable
	/// property set to true.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "repThreshDl": {
	///      "type": "integer"
	///    },
	///    "repThreshRp": {
	///      "type": "integer"
	///    },
	///    "repThreshUl": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringInformationRm(pub Option<QosMonitoringInformationRmInner>);

	impl ::std::ops::Deref for QosMonitoringInformationRm {
		type Target = Option<QosMonitoringInformationRmInner>;
		fn deref(&self) -> &Option<QosMonitoringInformationRmInner> {
			&self.0
		}
	}

	impl From<QosMonitoringInformationRm> for Option<QosMonitoringInformationRmInner> {
		fn from(value: QosMonitoringInformationRm) -> Self {
			value.0
		}
	}

	impl From<&QosMonitoringInformationRm> for QosMonitoringInformationRm {
		fn from(value: &QosMonitoringInformationRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<QosMonitoringInformationRmInner>> for QosMonitoringInformationRm {
		fn from(value: Option<QosMonitoringInformationRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the
	/// QosMonitoringInformation data type, but with the OpenAPI nullable
	/// property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// QosMonitoringInformation data type, but with the OpenAPI nullable
	/// property set to true.",
	///  "type": "object",
	///  "properties": {
	///    "repThreshDl": {
	///      "type": "integer"
	///    },
	///    "repThreshRp": {
	///      "type": "integer"
	///    },
	///    "repThreshUl": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringInformationRmInner {
		#[serde(
			rename = "repThreshDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_dl: Option<i64>,
		#[serde(
			rename = "repThreshRp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_rp: Option<i64>,
		#[serde(
			rename = "repThreshUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rep_thresh_ul: Option<i64>,
	}

	impl From<&QosMonitoringInformationRmInner> for QosMonitoringInformationRmInner {
		fn from(value: &QosMonitoringInformationRmInner) -> Self {
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
	///  "required": [
	///    "refPccRuleIds"
	///  ],
	///  "properties": {
	///    "dlDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "pdmf": {
	///      "description": "Represents the packet delay measurement failure
	/// indicator.",
	///      "type": "boolean"
	///    },
	///    "refPccRuleIds": {
	///      "description": "An array of PCC rule id references to the PCC rules
	/// associated with the QoS monitoring report.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "rtDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "ulDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
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
	pub struct QosMonitoringReport {
		#[serde(rename = "dlDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub dl_delays: Vec<i64>,
		/// Represents the packet delay measurement failure indicator.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pdmf: Option<bool>,
		/// An array of PCC rule id references to the PCC rules associated with
		/// the QoS monitoring report.
		#[serde(rename = "refPccRuleIds")]
		pub ref_pcc_rule_ids: Vec<String>,
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

	/// QoS Monitoring reporting information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS Monitoring reporting information.",
	///  "type": "object",
	///  "properties": {
	///    "dlDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
	///      },
	///      "minItems": 1
	///    },
	///    "pdmf": {
	///      "description": "Represents the packet delay measurement failure
	/// indicator.",
	///      "type": "boolean"
	///    },
	///    "rtDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "ulDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
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
	pub struct QosMonitoringReport1 {
		#[serde(rename = "dlDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub dl_delays: Vec<i64>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub flows: Vec<Flows>,
		/// Represents the packet delay measurement failure indicator.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pdmf: Option<bool>,
		#[serde(rename = "rtDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub rt_delays: Vec<i64>,
		#[serde(rename = "ulDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub ul_delays: Vec<i64>,
	}

	impl From<&QosMonitoringReport1> for QosMonitoringReport1 {
		fn from(value: &QosMonitoringReport1) -> Self {
			value.clone()
		}
	}

	/// Indicates the notification type for QoS Notification Control.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the notification type for QoS Notification
	/// Control.",
	///  "type": "string",
	///  "enum": [
	///    "GUARANTEED",
	///    "NOT_GUARANTEED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum QosNotifType {
		#[default]
		#[serde(rename = "GUARANTEED")]
		Guaranteed,
		#[serde(rename = "NOT_GUARANTEED")]
		NotGuaranteed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QosNotifType> for QosNotifType {
		fn from(value: &QosNotifType) -> Self {
			value.clone()
		}
	}

	impl ToString for QosNotifType {
		fn to_string(&self) -> String {
			match *self {
				Self::Guaranteed => "GUARANTEED".to_string(),
				Self::NotGuaranteed => "NOT_GUARANTEED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QosNotifType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GUARANTEED" => Ok(Self::Guaranteed),
				"NOT_GUARANTEED" => Ok(Self::NotGuaranteed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QosNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QosNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QosNotifType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the QoS Notification Control Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the QoS Notification Control Information.",
	///  "type": "object",
	///  "required": [
	///    "notifType",
	///    "refPccRuleIds"
	///  ],
	///  "properties": {
	///    "altQosParamId": {
	///      "type": "string"
	///    },
	///    "contVer": {
	///      "$ref": "#/components/schemas/ContentVersion"
	///    },
	///    "notifType": {
	///      "$ref": "#/components/schemas/QosNotifType"
	///    },
	///    "refPccRuleIds": {
	///      "description": "An array of PCC rule id references to the PCC rules
	/// associated with the QoS notification control info.",
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
	pub struct QosNotificationControlInfo {
		#[serde(
			rename = "altQosParamId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_qos_param_id: Option<String>,
		#[serde(rename = "contVer", default, skip_serializing_if = "Option::is_none")]
		pub cont_ver: Option<ContentVersion>,
		#[serde(rename = "notifType")]
		pub notif_type: QosNotifType,
		/// An array of PCC rule id references to the PCC rules associated with
		/// the QoS notification control info.
		#[serde(rename = "refPccRuleIds")]
		pub ref_pcc_rule_ids: Vec<String>,
	}

	impl From<&QosNotificationControlInfo> for QosNotificationControlInfo {
		fn from(value: &QosNotificationControlInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates whether the QoS targets for a GRB flow are not guaranteed or
	/// guaranteed again.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the QoS targets for a GRB flow are
	/// not guaranteed or guaranteed again.",
	///  "type": "object",
	///  "required": [
	///    "notifType"
	///  ],
	///  "properties": {
	///    "altSerReq": {
	///      "type": "string"
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
	///      },
	///      "minItems": 1
	///    },
	///    "notifType": {
	///      "$ref": "#/components/schemas/QosNotifType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosNotificationControlInfo1 {
		#[serde(rename = "altSerReq", default, skip_serializing_if = "Option::is_none")]
		pub alt_ser_req: Option<String>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub flows: Vec<Flows>,
		#[serde(rename = "notifType")]
		pub notif_type: QosNotifType,
	}

	impl From<&QosNotificationControlInfo1> for QosNotificationControlInfo1 {
		fn from(value: &QosNotificationControlInfo1) -> Self {
			value.clone()
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

	/// Possible values are
	/// - IPV4_ADDR: Indicates that the address type is in the form of
	///   "dotted-decimal" IPv4 address.
	/// - IPV6_ADDR: Indicates that the address type is in the form of IPv6
	///   address.
	/// - URL: Indicates that the address type is in the form of Uniform
	///   Resource Locator.
	/// - SIP_URI: Indicates that the address type is in the form of SIP Uniform
	///   Resource Identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- IPV4_ADDR: Indicates that the
	/// address type is in the form of \"dotted-decimal\" IPv4 address.\n-
	/// IPV6_ADDR: Indicates that the address type is in the form of IPv6
	/// address.\n- URL: Indicates that the address type is in the form of
	/// Uniform Resource Locator.\n- SIP_URI: Indicates that the address type is
	/// in the form of SIP Uniform Resource Identifier.\n",
	///  "type": "string",
	///  "enum": [
	///    "IPV4_ADDR",
	///    "IPV6_ADDR",
	///    "URL",
	///    "SIP_URI"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
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
		#[serde(rename = "IPV4_ADDR")]
		Ipv4Addr,
		#[serde(rename = "IPV6_ADDR")]
		Ipv6Addr,
		#[serde(rename = "URL")]
		Url,
		#[serde(rename = "SIP_URI")]
		SipUri,
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
				Self::Ipv4Addr => "IPV4_ADDR".to_string(),
				Self::Ipv6Addr => "IPV6_ADDR".to_string(),
				Self::Url => "URL".to_string(),
				Self::SipUri => "SIP_URI".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RedirectAddressType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IPV4_ADDR" => Ok(Self::Ipv4Addr),
				"IPV6_ADDR" => Ok(Self::Ipv6Addr),
				"URL" => Ok(Self::Url),
				"SIP_URI" => Ok(Self::SipUri),
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

	/// Contains the redirect information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the redirect information.",
	///  "type": "object",
	///  "properties": {
	///    "redirectAddressType": {
	///      "$ref": "#/components/schemas/RedirectAddressType"
	///    },
	///    "redirectEnabled": {
	///      "description": "Indicates the redirect is enable.",
	///      "type": "boolean"
	///    },
	///    "redirectServerAddress": {
	///      "description": "Indicates the address of the redirect server. If \"redirectAddressType\" attribute indicates the IPV4_ADDR, the encoding is the same as the Ipv4Addr data type defined in 3GPP TS 29.571.If \"redirectAddressType\" attribute indicates the IPV6_ADDR, the encoding is the same as the Ipv6Addr data type defined in 3GPP TS 29.571.If \"redirectAddressType\" attribute indicates the URL or SIP_URI, the encoding is the same as the Uri data type defined in 3GPP TS 29.571.\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedirectInformation {
		#[serde(
			rename = "redirectAddressType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redirect_address_type: Option<RedirectAddressType>,
		/// Indicates the redirect is enable.
		#[serde(
			rename = "redirectEnabled",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redirect_enabled: Option<bool>,
		/// Indicates the address of the redirect server. If
		/// "redirectAddressType" attribute indicates the IPV4_ADDR, the
		/// encoding is the same as the Ipv4Addr data type defined in 3GPP TS
		/// 29.571.If "redirectAddressType" attribute indicates the IPV6_ADDR,
		/// the encoding is the same as the Ipv6Addr data type defined in 3GPP
		/// TS 29.571.If "redirectAddressType" attribute indicates the URL or
		/// SIP_URI, the encoding is the same as the Uri data type defined in
		/// 3GPP TS 29.571.
		#[serde(
			rename = "redirectServerAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redirect_server_address: Option<String>,
	}

	impl From<&RedirectInformation> for RedirectInformation {
		fn from(value: &RedirectInformation) -> Self {
			value.clone()
		}
	}

	/// Indicates the frequency for the reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the frequency for the reporting.",
	///  "type": "string",
	///  "enum": [
	///    "EVENT_TRIGGERED",
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
	pub enum ReportingFrequency {
		#[default]
		#[serde(rename = "EVENT_TRIGGERED")]
		EventTriggered,
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportingFrequency> for ReportingFrequency {
		fn from(value: &ReportingFrequency) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportingFrequency {
		fn to_string(&self) -> String {
			match *self {
				Self::EventTriggered => "EVENT_TRIGGERED".to_string(),
				Self::Periodic => "PERIODIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportingFrequency {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EVENT_TRIGGERED" => Ok(Self::EventTriggered),
				"PERIODIC" => Ok(Self::Periodic),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportingFrequency {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingFrequency {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingFrequency {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
	///      "$ref": "#/components/schemas/NotificationMethod"
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
		pub notif_method: Option<NotificationMethod>,
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

	/// Possible values are
	/// - SER_ID_LEVEL: Indicates that the usage shall be reported on service id
	///   and rating group combination level.
	/// - RAT_GR_LEVEL: Indicates that the usage shall be reported on rating
	///   group level.
	/// - SPON_CON_LEVEL: Indicates that the usage shall be reported on sponsor
	///   identity and rating group combination level.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- SER_ID_LEVEL: Indicates that the
	/// usage shall be reported on service id and rating group combination
	/// level.\n- RAT_GR_LEVEL: Indicates that the usage shall be reported on
	/// rating group level.\n- SPON_CON_LEVEL: Indicates that the usage shall be
	/// reported on sponsor identity and rating group combination level.\n",
	///  "anyOf": [
	///    {
	///      "type": "string",
	///      "enum": [
	///        "SER_ID_LEVEL",
	///        "RAT_GR_LEVEL",
	///        "SPON_CON_LEVEL"
	///      ]
	///    },
	///    {
	///      "$ref": "#/components/schemas/NullValue"
	///    },
	///    {
	///      "description": "This string provides forward-compatibility with
	/// future extensions to the enumeration but is not used to encode content
	/// defined in the present version of this API.\n",
	///      "type": "string"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingLevel {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<ReportingLevelSubtype0>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<NullValue>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_2: Option<String>,
	}

	impl From<&ReportingLevel> for ReportingLevel {
		fn from(value: &ReportingLevel) -> Self {
			value.clone()
		}
	}

	/// ReportingLevelSubtype0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SER_ID_LEVEL",
	///    "RAT_GR_LEVEL",
	///    "SPON_CON_LEVEL"
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
	pub enum ReportingLevelSubtype0 {
		#[default]
		#[serde(rename = "SER_ID_LEVEL")]
		SerIdLevel,
		#[serde(rename = "RAT_GR_LEVEL")]
		RatGrLevel,
		#[serde(rename = "SPON_CON_LEVEL")]
		SponConLevel,
	}

	impl From<&ReportingLevelSubtype0> for ReportingLevelSubtype0 {
		fn from(value: &ReportingLevelSubtype0) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportingLevelSubtype0 {
		fn to_string(&self) -> String {
			match *self {
				Self::SerIdLevel => "SER_ID_LEVEL".to_string(),
				Self::RatGrLevel => "RAT_GR_LEVEL".to_string(),
				Self::SponConLevel => "SPON_CON_LEVEL".to_string(),
			}
		}
	}

	impl std::str::FromStr for ReportingLevelSubtype0 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SER_ID_LEVEL" => Ok(Self::SerIdLevel),
				"RAT_GR_LEVEL" => Ok(Self::RatGrLevel),
				"SPON_CON_LEVEL" => Ok(Self::SponConLevel),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportingLevelSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingLevelSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingLevelSubtype0 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - LOC_CH: Location change (tracking area). The tracking area of the UE
	///   has changed.
	/// - PRA_CH: Change of UE presence in PRA. The AMF reports the current
	///   presence status
	///  of the UE in a Presence Reporting Area, and notifies that the UE
	/// enters/leaves the  Presence Reporting Area.
	/// - SERV_AREA_CH: Service Area Restriction change. The UDM notifies the
	///   AMF that the
	///  subscribed service area restriction information has changed.
	/// - RFSP_CH: RFSP index change. The UDM notifies the AMF that the
	///   subscribed RFSP index has
	///  changed.
	/// - ALLOWED_NSSAI_CH: Allowed NSSAI change. The AMF notifies that the set
	///   of UE allowed
	///  S-NSSAIs has changed.
	/// - UE_AMBR_CH: UE-AMBR change. The UDM notifies the AMF that the
	///   subscribed UE-AMBR has
	///  changed.
	/// - SMF_SELECT_CH: SMF selection information change. The UE requested for
	///   an unsupported
	///  DNN or UE requested for a DNN within the list of DNN candidates for
	/// replacement per  S-NSSAI.
	/// - ACCESS_TYPE_CH: Access Type change. The AMF notifies that the access
	///   type and the RAT
	///  type combinations available in the AMF for a UE with simultaneous 3GPP
	/// and non-3GPP  connectivity has changed.
	/// - UE_SLICE_MBR_CH: UE-Slice-MBR change. The NF service consumer notifies
	///   any changes
	///  in the subscribed UE-Slice-MBR for each subscribed S-NSSAI of the home
	/// PLMN mapping  to a S-NSSAI of the serving PLMN.
	/// - NWDAF_DATA_CH: NDWAF DATA CHANGE. The AMF notifies that the NWDAF
	///   instance IDs used
	///  for the UE and/or associated Analytics IDs used for the UE and
	/// available in the AMF  have changed.
	/// - TARGET_NSSAI: Generation of Target NSSAI. The NF service consumer
	///   notifies that the
	///  Target NSSAI was generated.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- LOC_CH: Location change
	/// (tracking area). The tracking area of the UE has changed.\n- PRA_CH:
	/// Change of UE presence in PRA. The AMF reports the current presence
	/// status\n  of the UE in a Presence Reporting Area, and notifies that the
	/// UE enters/leaves the\n  Presence Reporting Area.\n- SERV_AREA_CH:
	/// Service Area Restriction change. The UDM notifies the AMF that the\n
	/// subscribed service area restriction information has changed.\n- RFSP_CH:
	/// RFSP index change. The UDM notifies the AMF that the subscribed RFSP
	/// index has\n  changed.\n- ALLOWED_NSSAI_CH: Allowed NSSAI change. The AMF
	/// notifies that the set of UE allowed\n  S-NSSAIs has changed.\n-
	/// UE_AMBR_CH: UE-AMBR change. The UDM notifies the AMF that the subscribed
	/// UE-AMBR has\n  changed.\n- SMF_SELECT_CH: SMF selection information
	/// change. The UE requested for an unsupported\n  DNN or UE requested for a
	/// DNN within the list of DNN candidates for replacement per\n  S-NSSAI.\n-
	/// ACCESS_TYPE_CH: Access Type change. The AMF notifies that the access
	/// type and the RAT\n  type combinations available in the AMF for a UE with
	/// simultaneous 3GPP and non-3GPP\n  connectivity has changed. \n-
	/// UE_SLICE_MBR_CH: UE-Slice-MBR change. The NF service consumer notifies
	/// any changes \n  in the subscribed UE-Slice-MBR for each subscribed
	/// S-NSSAI of the home PLMN mapping \n  to a S-NSSAI of the serving
	/// PLMN.\n- NWDAF_DATA_CH: NDWAF DATA CHANGE. The AMF notifies that the
	/// NWDAF instance IDs used\n  for the UE and/or associated Analytics IDs
	/// used for the UE and available in the AMF\n  have changed.\n-
	/// TARGET_NSSAI: Generation of Target NSSAI. The NF service consumer
	/// notifies that the\n  Target NSSAI was generated.\n",
	///  "type": "string",
	///  "enum": [
	///    "LOC_CH",
	///    "PRA_CH",
	///    "SERV_AREA_CH",
	///    "RFSP_CH",
	///    "ALLOWED_NSSAI_CH",
	///    "UE_AMBR_CH",
	///    "UE_SLICE_MBR_CH",
	///    "SMF_SELECT_CH",
	///    "ACCESS_TYPE_CH",
	///    "NWDAF_DATA_CH",
	///    "TARGET_NSSAI"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RequestTrigger {
		#[default]
		#[serde(rename = "LOC_CH")]
		LocCh,
		#[serde(rename = "PRA_CH")]
		PraCh,
		#[serde(rename = "SERV_AREA_CH")]
		ServAreaCh,
		#[serde(rename = "RFSP_CH")]
		RfspCh,
		#[serde(rename = "ALLOWED_NSSAI_CH")]
		AllowedNssaiCh,
		#[serde(rename = "UE_AMBR_CH")]
		UeAmbrCh,
		#[serde(rename = "UE_SLICE_MBR_CH")]
		UeSliceMbrCh,
		#[serde(rename = "SMF_SELECT_CH")]
		SmfSelectCh,
		#[serde(rename = "ACCESS_TYPE_CH")]
		AccessTypeCh,
		#[serde(rename = "NWDAF_DATA_CH")]
		NwdafDataCh,
		#[serde(rename = "TARGET_NSSAI")]
		TargetNssai,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestTrigger> for RequestTrigger {
		fn from(value: &RequestTrigger) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestTrigger {
		fn to_string(&self) -> String {
			match *self {
				Self::LocCh => "LOC_CH".to_string(),
				Self::PraCh => "PRA_CH".to_string(),
				Self::ServAreaCh => "SERV_AREA_CH".to_string(),
				Self::RfspCh => "RFSP_CH".to_string(),
				Self::AllowedNssaiCh => "ALLOWED_NSSAI_CH".to_string(),
				Self::UeAmbrCh => "UE_AMBR_CH".to_string(),
				Self::UeSliceMbrCh => "UE_SLICE_MBR_CH".to_string(),
				Self::SmfSelectCh => "SMF_SELECT_CH".to_string(),
				Self::AccessTypeCh => "ACCESS_TYPE_CH".to_string(),
				Self::NwdafDataCh => "NWDAF_DATA_CH".to_string(),
				Self::TargetNssai => "TARGET_NSSAI".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestTrigger {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOC_CH" => Ok(Self::LocCh),
				"PRA_CH" => Ok(Self::PraCh),
				"SERV_AREA_CH" => Ok(Self::ServAreaCh),
				"RFSP_CH" => Ok(Self::RfspCh),
				"ALLOWED_NSSAI_CH" => Ok(Self::AllowedNssaiCh),
				"UE_AMBR_CH" => Ok(Self::UeAmbrCh),
				"UE_SLICE_MBR_CH" => Ok(Self::UeSliceMbrCh),
				"SMF_SELECT_CH" => Ok(Self::SmfSelectCh),
				"ACCESS_TYPE_CH" => Ok(Self::AccessTypeCh),
				"NWDAF_DATA_CH" => Ok(Self::NwdafDataCh),
				"TARGET_NSSAI" => Ok(Self::TargetNssai),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - LOC_CH: Location change (tracking area). The tracking area of the UE
	///   has changed.
	/// - PRA_CH: Change of UE presence in PRA. The AMF reports the current
	///   presence status of the UE
	///  in a Presence Reporting Area, and notifies that the UE enters/leaves
	/// the Presence Reporting  Area.
	/// - UE_POLICY: A MANAGE UE POLICY COMPLETE message or a MANAGE UE POLICY
	///   COMMAND REJECT
	///  message, as defined in Annex D.5 of 3GPP TS 24.501 or a "UE POLICY
	/// PROVISIONING REQUEST"  message, as defined in clause 7.2.1.1 of 3GPP
	/// TS 24.587 , has been received by the AMF  and is being forwarded.
	/// - PLMN_CH: PLMN change. the serving PLMN of UE has changed.
	/// - CON_STATE_CH: Connectivity state change: the connectivity state of UE
	///   has changed.
	/// - GROUP_ID_LIST_CHG: UE Internal Group Identifier(s) has changed. This
	///   policy control request
	///  trigger does not require a subscription
	/// - UE_CAP_CH: UE Capabilities change: the UE provided 5G ProSe
	///   capabilities have changed.
	///  This policy control request trigger does not require subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- LOC_CH: Location change
	/// (tracking area). The tracking area of the UE has changed.\n- PRA_CH:
	/// Change of UE presence in PRA. The AMF reports the current presence
	/// status of the UE\n  in a Presence Reporting Area, and notifies that the
	/// UE enters/leaves the Presence Reporting\n  Area.\n- UE_POLICY: A MANAGE
	/// UE POLICY COMPLETE message or a MANAGE UE POLICY COMMAND REJECT\n
	/// message, as defined in Annex D.5 of 3GPP TS 24.501 or a \"UE POLICY
	/// PROVISIONING REQUEST\"\n  message, as defined in clause 7.2.1.1 of 3GPP
	/// TS 24.587 , has been received by the AMF\n  and is being forwarded.\n-
	/// PLMN_CH: PLMN change. the serving PLMN of UE has changed. \n-
	/// CON_STATE_CH: Connectivity state change: the connectivity state of UE
	/// has changed. \n- GROUP_ID_LIST_CHG: UE Internal Group Identifier(s) has
	/// changed. This policy control request\n  trigger does not require a
	/// subscription\n- UE_CAP_CH: UE Capabilities change: the UE provided 5G
	/// ProSe capabilities have changed.\n  This policy control request trigger
	/// does not require subscription. \n",
	///  "type": "string",
	///  "enum": [
	///    "LOC_CH",
	///    "PRA_CH",
	///    "UE_POLICY",
	///    "PLMN_CH",
	///    "CON_STATE_CH",
	///    "GROUP_ID_LIST_CHG",
	///    "UE_CAP_CH"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RequestTrigger1 {
		#[default]
		#[serde(rename = "LOC_CH")]
		LocCh,
		#[serde(rename = "PRA_CH")]
		PraCh,
		#[serde(rename = "UE_POLICY")]
		UePolicy,
		#[serde(rename = "PLMN_CH")]
		PlmnCh,
		#[serde(rename = "CON_STATE_CH")]
		ConStateCh,
		#[serde(rename = "GROUP_ID_LIST_CHG")]
		GroupIdListChg,
		#[serde(rename = "UE_CAP_CH")]
		UeCapCh,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestTrigger1> for RequestTrigger1 {
		fn from(value: &RequestTrigger1) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestTrigger1 {
		fn to_string(&self) -> String {
			match *self {
				Self::LocCh => "LOC_CH".to_string(),
				Self::PraCh => "PRA_CH".to_string(),
				Self::UePolicy => "UE_POLICY".to_string(),
				Self::PlmnCh => "PLMN_CH".to_string(),
				Self::ConStateCh => "CON_STATE_CH".to_string(),
				Self::GroupIdListChg => "GROUP_ID_LIST_CHG".to_string(),
				Self::UeCapCh => "UE_CAP_CH".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestTrigger1 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOC_CH" => Ok(Self::LocCh),
				"PRA_CH" => Ok(Self::PraCh),
				"UE_POLICY" => Ok(Self::UePolicy),
				"PLMN_CH" => Ok(Self::PlmnCh),
				"CON_STATE_CH" => Ok(Self::ConStateCh),
				"GROUP_ID_LIST_CHG" => Ok(Self::GroupIdListChg),
				"UE_CAP_CH" => Ok(Self::UeCapCh),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestTrigger1 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestTrigger1 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestTrigger1 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the QoS information requested by the UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the QoS information requested by the UE.",
	///  "type": "object",
	///  "required": [
	///    "5qi"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "gbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RequestedQos {
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(rename = "gbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_dl: Option<BitRate>,
		#[serde(rename = "gbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_ul: Option<BitRate>,
	}

	impl From<&RequestedQos> for RequestedQos {
		fn from(value: &RequestedQos) -> Self {
			value.clone()
		}
	}

	/// Indicates the requested QoS monitoring parameters to be measured.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the requested QoS monitoring parameters to be
	/// measured.",
	///  "type": "string",
	///  "enum": [
	///    "DOWNLINK",
	///    "UPLINK",
	///    "ROUND_TRIP"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RequestedQosMonitoringParameter {
		#[default]
		#[serde(rename = "DOWNLINK")]
		Downlink,
		#[serde(rename = "UPLINK")]
		Uplink,
		#[serde(rename = "ROUND_TRIP")]
		RoundTrip,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestedQosMonitoringParameter> for RequestedQosMonitoringParameter {
		fn from(value: &RequestedQosMonitoringParameter) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestedQosMonitoringParameter {
		fn to_string(&self) -> String {
			match *self {
				Self::Downlink => "DOWNLINK".to_string(),
				Self::Uplink => "UPLINK".to_string(),
				Self::RoundTrip => "ROUND_TRIP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestedQosMonitoringParameter {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DOWNLINK" => Ok(Self::Downlink),
				"UPLINK" => Ok(Self::Uplink),
				"ROUND_TRIP" => Ok(Self::RoundTrip),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestedQosMonitoringParameter {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestedQosMonitoringParameter {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestedQosMonitoringParameter {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains rule data requested by the PCF to receive information
	/// associated with PCC rule(s).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains rule data requested by the PCF to receive
	/// information associated with PCC rule(s).",
	///  "type": "object",
	///  "required": [
	///    "refPccRuleIds",
	///    "reqData"
	///  ],
	///  "properties": {
	///    "refPccRuleIds": {
	///      "description": "An array of PCC rule id references to the PCC rules
	/// associated with the control data.",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "reqData": {
	///      "description": "Array of requested rule data type elements
	/// indicating what type of rule data is requested for the corresponding
	/// referenced PCC rules.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedRuleDataType"
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
	pub struct RequestedRuleData {
		/// An array of PCC rule id references to the PCC rules associated with
		/// the control data.
		#[serde(rename = "refPccRuleIds")]
		pub ref_pcc_rule_ids: Vec<String>,
		/// Array of requested rule data type elements indicating what type of
		/// rule data is requested for the corresponding referenced PCC rules.
		#[serde(rename = "reqData")]
		pub req_data: Vec<RequestedRuleDataType>,
	}

	impl From<&RequestedRuleData> for RequestedRuleData {
		fn from(value: &RequestedRuleData) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - CH_ID: Indicates that the requested rule data is the charging
	///   identifier.
	/// - MS_TIME_ZONE: Indicates that the requested access network info type is
	///   the UE's timezone.
	/// - USER_LOC_INFO: Indicates that the requested access network info type
	///   is the UE's location.
	/// - RES_RELEASE: Indicates that the requested rule data is the result of
	///   the release of resource.
	/// - SUCC_RES_ALLO: Indicates that the requested rule data is the
	///   successful resource allocation.
	/// - EPS_FALLBACK: Indicates that the requested rule data is the report of
	///   QoS flow rejection due to EPS fallback.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- CH_ID: Indicates that the
	/// requested rule data is the charging identifier. \n- MS_TIME_ZONE:
	/// Indicates that the requested access network info type is the UE's
	/// timezone.\n- USER_LOC_INFO: Indicates that the requested access network
	/// info type is the UE's location.\n- RES_RELEASE: Indicates that the
	/// requested rule data is the result of the release of resource.\n-
	/// SUCC_RES_ALLO: Indicates that the requested rule data is the successful
	/// resource allocation.\n- EPS_FALLBACK: Indicates that the requested rule
	/// data is the report of QoS flow rejection due to EPS fallback.\n",
	///  "type": "string",
	///  "enum": [
	///    "CH_ID",
	///    "MS_TIME_ZONE",
	///    "USER_LOC_INFO",
	///    "RES_RELEASE",
	///    "SUCC_RES_ALLO",
	///    "EPS_FALLBACK"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RequestedRuleDataType {
		#[default]
		#[serde(rename = "CH_ID")]
		ChId,
		#[serde(rename = "MS_TIME_ZONE")]
		MsTimeZone,
		#[serde(rename = "USER_LOC_INFO")]
		UserLocInfo,
		#[serde(rename = "RES_RELEASE")]
		ResRelease,
		#[serde(rename = "SUCC_RES_ALLO")]
		SuccResAllo,
		#[serde(rename = "EPS_FALLBACK")]
		EpsFallback,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestedRuleDataType> for RequestedRuleDataType {
		fn from(value: &RequestedRuleDataType) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestedRuleDataType {
		fn to_string(&self) -> String {
			match *self {
				Self::ChId => "CH_ID".to_string(),
				Self::MsTimeZone => "MS_TIME_ZONE".to_string(),
				Self::UserLocInfo => "USER_LOC_INFO".to_string(),
				Self::ResRelease => "RES_RELEASE".to_string(),
				Self::SuccResAllo => "SUCC_RES_ALLO".to_string(),
				Self::EpsFallback => "EPS_FALLBACK".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestedRuleDataType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CH_ID" => Ok(Self::ChId),
				"MS_TIME_ZONE" => Ok(Self::MsTimeZone),
				"USER_LOC_INFO" => Ok(Self::UserLocInfo),
				"RES_RELEASE" => Ok(Self::ResRelease),
				"SUCC_RES_ALLO" => Ok(Self::SuccResAllo),
				"EPS_FALLBACK" => Ok(Self::EpsFallback),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestedRuleDataType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestedRuleDataType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestedRuleDataType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains usage data requested by the PCF requesting usage reports for
	/// the corresponding usage monitoring data instances.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains usage data requested by the PCF requesting usage reports for the corresponding usage monitoring data instances.",
	///  "type": "object",
	///  "properties": {
	///    "allUmIds": {
	///      "description": "This boolean indicates whether requested usage data
	/// applies to all usage monitoring data instances. When it's not included,
	/// it means requested usage data shall only apply to the usage monitoring
	/// data instances referenced by the refUmIds attribute.\n",
	///      "type": "boolean"
	///    },
	///    "refUmIds": {
	///      "description": "An array of usage monitoring data id references to
	/// the usage monitoring data instances for which the PCF is requesting a
	/// usage report. This attribute shall only be provided when allUmIds is not
	/// set to true.\n",
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
	pub struct RequestedUsageData {
		/// This boolean indicates whether requested usage data applies to all
		/// usage monitoring data instances. When it's not included, it means
		/// requested usage data shall only apply to the usage monitoring data
		/// instances referenced by the refUmIds attribute.
		#[serde(rename = "allUmIds", default, skip_serializing_if = "Option::is_none")]
		pub all_um_ids: Option<bool>,
		/// An array of usage monitoring data id references to the usage
		/// monitoring data instances for which the PCF is requesting a usage
		/// report. This attribute shall only be provided when allUmIds is not
		/// set to true.
		#[serde(rename = "refUmIds", default, skip_serializing_if = "Vec::is_empty")]
		pub ref_um_ids: Vec<String>,
	}

	impl From<&RequestedUsageData> for RequestedUsageData {
		fn from(value: &RequestedUsageData) -> Self {
			value.clone()
		}
	}

	/// Indicates the access network information required for an AF session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the access network information required for
	/// an AF session.",
	///  "type": "string",
	///  "enum": [
	///    "USER_LOCATION",
	///    "UE_TIME_ZONE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RequiredAccessInfo {
		#[default]
		#[serde(rename = "USER_LOCATION")]
		UserLocation,
		#[serde(rename = "UE_TIME_ZONE")]
		UeTimeZone,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequiredAccessInfo> for RequiredAccessInfo {
		fn from(value: &RequiredAccessInfo) -> Self {
			value.clone()
		}
	}

	impl ToString for RequiredAccessInfo {
		fn to_string(&self) -> String {
			match *self {
				Self::UserLocation => "USER_LOCATION".to_string(),
				Self::UeTimeZone => "UE_TIME_ZONE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequiredAccessInfo {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"USER_LOCATION" => Ok(Self::UserLocation),
				"UE_TIME_ZONE" => Ok(Self::UeTimeZone),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequiredAccessInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequiredAccessInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequiredAccessInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the status of the PCC rule(s) related to certain media
	/// components.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the status of the PCC rule(s) related to
	/// certain media components.",
	///  "type": "object",
	///  "properties": {
	///    "altSerReq": {
	///      "type": "string"
	///    },
	///    "flows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Flows"
	///      },
	///      "minItems": 1
	///    },
	///    "mcResourcStatus": {
	///      "$ref": "#/components/schemas/MediaComponentResourcesStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ResourcesAllocationInfo {
		#[serde(rename = "altSerReq", default, skip_serializing_if = "Option::is_none")]
		pub alt_ser_req: Option<String>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub flows: Vec<Flows>,
		#[serde(
			rename = "mcResourcStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mc_resourc_status: Option<MediaComponentResourcesStatus>,
	}

	impl From<&ResourcesAllocationInfo> for ResourcesAllocationInfo {
		fn from(value: &ResourcesAllocationInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - CREATE_PCC_RULE: Indicates to create a new PCC rule to reserve the
	///   resource requested by the UE.
	/// - DELETE_PCC_RULE: Indicates to delete a PCC rule corresponding to
	///   reserve the resource requested by the UE.
	/// - MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS: Indicates to modify the PCC
	///   rule by adding new packet filter(s).
	/// - MODIFY_ PCC_RULE_AND_REPLACE_PACKET_FILTERS: Indicates to modify the
	///   PCC rule by replacing the existing packet filter(s).
	/// - MODIFY_ PCC_RULE_AND_DELETE_PACKET_FILTERS: Indicates to modify the
	///   PCC rule by deleting the existing packet filter(s).
	/// - MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS: Indicates to modify the
	///   PCC rule by modifying the QoS of the PCC rule.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- CREATE_PCC_RULE: Indicates to
	/// create a new PCC rule to reserve the resource requested by the UE. \n-
	/// DELETE_PCC_RULE: Indicates to delete a PCC rule corresponding to reserve
	/// the resource requested by the UE.\n-
	/// MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS: Indicates to modify the PCC rule
	/// by adding new packet filter(s).\n- MODIFY_
	/// PCC_RULE_AND_REPLACE_PACKET_FILTERS: Indicates to modify the PCC rule by
	/// replacing the existing packet filter(s).\n- MODIFY_
	/// PCC_RULE_AND_DELETE_PACKET_FILTERS: Indicates to modify the PCC rule by
	/// deleting the existing packet filter(s).\n-
	/// MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS: Indicates to modify the
	/// PCC rule by modifying the QoS of the PCC rule.\n",
	///  "type": "string",
	///  "enum": [
	///    "CREATE_PCC_RULE",
	///    "DELETE_PCC_RULE",
	///    "MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS",
	///    "MODIFY_ PCC_RULE_AND_REPLACE_PACKET_FILTERS",
	///    "MODIFY_ PCC_RULE_AND_DELETE_PACKET_FILTERS",
	///    "MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RuleOperation {
		#[default]
		#[serde(rename = "CREATE_PCC_RULE")]
		CreatePccRule,
		#[serde(rename = "DELETE_PCC_RULE")]
		DeletePccRule,
		#[serde(rename = "MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS")]
		ModifyPccRuleAndAddPacketFilters,
		#[serde(rename = "MODIFY_ PCC_RULE_AND_REPLACE_PACKET_FILTERS")]
		ModifyPccRuleAndReplacePacketFilters,
		#[serde(rename = "MODIFY_ PCC_RULE_AND_DELETE_PACKET_FILTERS")]
		ModifyPccRuleAndDeletePacketFilters,
		#[serde(rename = "MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS")]
		ModifyPccRuleWithoutModifyPacketFilters,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RuleOperation> for RuleOperation {
		fn from(value: &RuleOperation) -> Self {
			value.clone()
		}
	}

	impl ToString for RuleOperation {
		fn to_string(&self) -> String {
			match *self {
				Self::CreatePccRule => "CREATE_PCC_RULE".to_string(),
				Self::DeletePccRule => "DELETE_PCC_RULE".to_string(),
				Self::ModifyPccRuleAndAddPacketFilters => {
					"MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS".to_string()
				}
				Self::ModifyPccRuleAndReplacePacketFilters => {
					"MODIFY_ PCC_RULE_AND_REPLACE_PACKET_FILTERS".to_string()
				}
				Self::ModifyPccRuleAndDeletePacketFilters => {
					"MODIFY_ PCC_RULE_AND_DELETE_PACKET_FILTERS".to_string()
				}
				Self::ModifyPccRuleWithoutModifyPacketFilters => {
					"MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RuleOperation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CREATE_PCC_RULE" => Ok(Self::CreatePccRule),
				"DELETE_PCC_RULE" => Ok(Self::DeletePccRule),
				"MODIFY_PCC_RULE_AND_ADD_PACKET_FILTERS" => {
					Ok(Self::ModifyPccRuleAndAddPacketFilters)
				}
				"MODIFY_ PCC_RULE_AND_REPLACE_PACKET_FILTERS" => {
					Ok(Self::ModifyPccRuleAndReplacePacketFilters)
				}
				"MODIFY_ PCC_RULE_AND_DELETE_PACKET_FILTERS" => {
					Ok(Self::ModifyPccRuleAndDeletePacketFilters)
				}
				"MODIFY_PCC_RULE_WITHOUT_MODIFY_PACKET_FILTERS" => {
					Ok(Self::ModifyPccRuleWithoutModifyPacketFilters)
				}
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RuleOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RuleOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RuleOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Reports the status of PCC.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Reports the status of PCC.",
	///  "type": "object",
	///  "required": [
	///    "pccRuleIds",
	///    "ruleStatus"
	///  ],
	///  "properties": {
	///    "altQosParamId": {
	///      "type": "string"
	///    },
	///    "contVers": {
	///      "description": "Indicates the version of a PCC rule.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ContentVersion"
	///      },
	///      "minItems": 1
	///    },
	///    "failureCode": {
	///      "$ref": "#/components/schemas/FailureCode"
	///    },
	///    "finUnitAct": {
	///      "$ref": "#/components/schemas/FinalUnitAction"
	///    },
	///    "pccRuleIds": {
	///      "description": "Contains the identifier of the affected PCC
	/// rule(s).",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ranNasRelCauses": {
	///      "description": "indicates the RAN or NAS release cause code
	/// information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RanNasRelCause"
	///      },
	///      "minItems": 1
	///    },
	///    "ruleStatus": {
	///      "$ref": "#/components/schemas/RuleStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RuleReport {
		#[serde(
			rename = "altQosParamId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_qos_param_id: Option<String>,
		/// Indicates the version of a PCC rule.
		#[serde(rename = "contVers", default, skip_serializing_if = "Vec::is_empty")]
		pub cont_vers: Vec<ContentVersion>,
		#[serde(
			rename = "failureCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failure_code: Option<FailureCode>,
		#[serde(
			rename = "finUnitAct",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub fin_unit_act: Option<FinalUnitAction>,
		/// Contains the identifier of the affected PCC rule(s).
		#[serde(rename = "pccRuleIds")]
		pub pcc_rule_ids: Vec<String>,
		/// indicates the RAN or NAS release cause code information.
		#[serde(
			rename = "ranNasRelCauses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ran_nas_rel_causes: Vec<RanNasRelCause>,
		#[serde(rename = "ruleStatus")]
		pub rule_status: RuleStatus,
	}

	impl From<&RuleReport> for RuleReport {
		fn from(value: &RuleReport) -> Self {
			value.clone()
		}
	}

	/// Possible values are
	/// - ACTIVE: Indicates that the PCC rule(s) are successfully installed (for
	///   those provisioned from PCF) or activated (for those pre-defined in
	///   SMF), or the session rule(s) are successfully installed
	/// - INACTIVE: Indicates that the PCC rule(s) are removed (for those
	///   provisioned from PCF) or inactive (for those pre-defined in SMF) or
	///   the session rule(s) are removed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- ACTIVE: Indicates that the PCC
	/// rule(s) are successfully installed (for those provisioned from PCF) or
	/// activated (for those pre-defined in SMF), or the session rule(s) are
	/// successfully installed \n- INACTIVE: Indicates that the PCC rule(s) are
	/// removed (for those provisioned from PCF) or inactive (for those
	/// pre-defined in SMF) or the session rule(s) are removed.\n",
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
	pub enum RuleStatus {
		#[default]
		#[serde(rename = "ACTIVE")]
		Active,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RuleStatus> for RuleStatus {
		fn from(value: &RuleStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for RuleStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Active => "ACTIVE".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RuleStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE" => Ok(Self::Active),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RuleStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the address of the access network gateway control node.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the address of the access network gateway
	/// control node.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "anGwIpv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "anGwIpv6Addr"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "anGwIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "anGwIpv6Addr": {
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
	pub enum SchemasAnGwAddress {
		#[default]
		Variant0 {
			#[serde(rename = "anGwIpv4Addr")]
			an_gw_ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "anGwIpv6Addr")]
			an_gw_ipv6_addr: Ipv6Addr,
		},
	}

	impl From<&SchemasAnGwAddress> for SchemasAnGwAddress {
		fn from(value: &SchemasAnGwAddress) -> Self {
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

	/// Unsigned integer identifying a period of time in units of seconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a period of time in units
	/// of seconds.",
	///  "type": "integer",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDurationSec(pub u64);

	impl ::std::ops::Deref for SchemasDurationSec {
		type Target = u64;
		fn deref(&self) -> &u64 {
			&self.0
		}
	}

	impl From<SchemasDurationSec> for u64 {
		fn from(value: SchemasDurationSec) -> Self {
			value.0
		}
	}

	impl From<&SchemasDurationSec> for SchemasDurationSec {
		fn from(value: &SchemasDurationSec) -> Self {
			value.clone()
		}
	}

	impl From<u64> for SchemasDurationSec {
		fn from(value: u64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasDurationSec {
		type Err = <u64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SchemasDurationSec {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasDurationSec {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasDurationSec {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SchemasDurationSec {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned integer identifying a period of time in units of seconds with
	/// "nullable=true" property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a period of time in units
	/// of seconds with \"nullable=true\" property.",
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
	pub struct SchemasDurationSecRm(pub Option<u64>);

	impl ::std::ops::Deref for SchemasDurationSecRm {
		type Target = Option<u64>;
		fn deref(&self) -> &Option<u64> {
			&self.0
		}
	}

	impl From<SchemasDurationSecRm> for Option<u64> {
		fn from(value: SchemasDurationSecRm) -> Self {
			value.0
		}
	}

	impl From<&SchemasDurationSecRm> for SchemasDurationSecRm {
		fn from(value: &SchemasDurationSecRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u64>> for SchemasDurationSecRm {
		fn from(value: Option<u64>) -> Self {
			Self(value)
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
	///      "$ref": "#/components/schemas/schemas-FlowDescription"
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
	pub struct SchemasEthFlowDescription {
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
		pub f_desc: Option<SchemasFlowDescription>,
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

	impl From<&SchemasEthFlowDescription> for SchemasEthFlowDescription {
		fn from(value: &SchemasEthFlowDescription) -> Self {
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
	pub struct SchemasFlowDescription(pub String);

	impl ::std::ops::Deref for SchemasFlowDescription {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasFlowDescription> for String {
		fn from(value: SchemasFlowDescription) -> Self {
			value.0
		}
	}

	impl From<&SchemasFlowDescription> for SchemasFlowDescription {
		fn from(value: &SchemasFlowDescription) -> Self {
			value.clone()
		}
	}

	impl From<String> for SchemasFlowDescription {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasFlowDescription {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SchemasFlowDescription {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates the list of Analytic ID(s) per NWDAF instance ID used for the
	/// PDU Session consumed by the SMF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the list of Analytic ID(s) per NWDAF instance
	/// ID used for the PDU Session consumed by the SMF.",
	///  "type": "object",
	///  "required": [
	///    "nwdafInstanceId"
	///  ],
	///  "properties": {
	///    "nwdafEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "nwdafInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasNwdafData {
		#[serde(rename = "nwdafEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_events: Vec<NwdafEvent>,
		#[serde(rename = "nwdafInstanceId")]
		pub nwdaf_instance_id: NfInstanceId,
	}

	impl From<&SchemasNwdafData> for SchemasNwdafData {
		fn from(value: &SchemasNwdafData) -> Self {
			value.clone()
		}
	}

	/// It represents a list of Tracking Areas within a serving network.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It represents a list of Tracking Areas within a serving
	/// network.",
	///  "type": "object",
	///  "required": [
	///    "tacList"
	///  ],
	///  "properties": {
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "tacList": {
	///      "description": "Indicates a list of Tracking Areas where the
	/// service is allowed.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tac"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasServiceAreaCoverageInfo {
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		/// Indicates a list of Tracking Areas where the service is allowed.
		#[serde(rename = "tacList")]
		pub tac_list: Vec<Tac>,
	}

	impl From<&SchemasServiceAreaCoverageInfo> for SchemasServiceAreaCoverageInfo {
		fn from(value: &SchemasServiceAreaCoverageInfo) -> Self {
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
	pub enum SchemasTransportProtocol {
		#[default]
		#[serde(rename = "TCP")]
		Tcp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SchemasTransportProtocol> for SchemasTransportProtocol {
		fn from(value: &SchemasTransportProtocol) -> Self {
			value.clone()
		}
	}

	impl ToString for SchemasTransportProtocol {
		fn to_string(&self) -> String {
			match *self {
				Self::Tcp => "TCP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SchemasTransportProtocol {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TCP" => Ok(Self::Tcp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SchemasTransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasTransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasTransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates TSC Traffic pattern.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates TSC Traffic pattern.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "burstArrivalTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "periodicity": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInNumMsg": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInTime": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasTscaiInputContainer(pub Option<SchemasTscaiInputContainerInner>);

	impl ::std::ops::Deref for SchemasTscaiInputContainer {
		type Target = Option<SchemasTscaiInputContainerInner>;
		fn deref(&self) -> &Option<SchemasTscaiInputContainerInner> {
			&self.0
		}
	}

	impl From<SchemasTscaiInputContainer> for Option<SchemasTscaiInputContainerInner> {
		fn from(value: SchemasTscaiInputContainer) -> Self {
			value.0
		}
	}

	impl From<&SchemasTscaiInputContainer> for SchemasTscaiInputContainer {
		fn from(value: &SchemasTscaiInputContainer) -> Self {
			value.clone()
		}
	}

	impl From<Option<SchemasTscaiInputContainerInner>> for SchemasTscaiInputContainer {
		fn from(value: Option<SchemasTscaiInputContainerInner>) -> Self {
			Self(value)
		}
	}

	/// Indicates TSC Traffic pattern.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates TSC Traffic pattern.",
	///  "type": "object",
	///  "properties": {
	///    "burstArrivalTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "periodicity": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInNumMsg": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInTime": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasTscaiInputContainerInner {
		#[serde(
			rename = "burstArrivalTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub burst_arrival_time: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub periodicity: Option<Uinteger>,
		#[serde(
			rename = "surTimeInNumMsg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sur_time_in_num_msg: Option<Uinteger>,
		#[serde(
			rename = "surTimeInTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sur_time_in_time: Option<Uinteger>,
	}

	impl From<&SchemasTscaiInputContainerInner> for SchemasTscaiInputContainerInner {
		fn from(value: &SchemasTscaiInputContainerInner) -> Self {
			value.clone()
		}
	}

	/// Indicates the result of the Policy Authorization service request from
	/// the AF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the result of the Policy Authorization
	/// service request from the AF.",
	///  "type": "string",
	///  "enum": [
	///    "TP_NOT_KNOWN",
	///    "TP_EXPIRED",
	///    "TP_NOT_YET_OCURRED",
	///    "ROUT_REQ_NOT_AUTHORIZED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ServAuthInfo {
		#[default]
		#[serde(rename = "TP_NOT_KNOWN")]
		TpNotKnown,
		#[serde(rename = "TP_EXPIRED")]
		TpExpired,
		#[serde(rename = "TP_NOT_YET_OCURRED")]
		TpNotYetOcurred,
		#[serde(rename = "ROUT_REQ_NOT_AUTHORIZED")]
		RoutReqNotAuthorized,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ServAuthInfo> for ServAuthInfo {
		fn from(value: &ServAuthInfo) -> Self {
			value.clone()
		}
	}

	impl ToString for ServAuthInfo {
		fn to_string(&self) -> String {
			match *self {
				Self::TpNotKnown => "TP_NOT_KNOWN".to_string(),
				Self::TpExpired => "TP_EXPIRED".to_string(),
				Self::TpNotYetOcurred => "TP_NOT_YET_OCURRED".to_string(),
				Self::RoutReqNotAuthorized => "ROUT_REQ_NOT_AUTHORIZED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ServAuthInfo {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TP_NOT_KNOWN" => Ok(Self::TpNotKnown),
				"TP_EXPIRED" => Ok(Self::TpExpired),
				"TP_NOT_YET_OCURRED" => Ok(Self::TpNotYetOcurred),
				"ROUT_REQ_NOT_AUTHORIZED" => Ok(Self::RoutReqNotAuthorized),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServAuthInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServAuthInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServAuthInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// It represents a list of Tracking Areas within a serving network.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It represents a list of Tracking Areas within a serving
	/// network.",
	///  "type": "object",
	///  "required": [
	///    "tacList"
	///  ],
	///  "properties": {
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "tacList": {
	///      "description": "Indicates a list of Tracking Areas where the
	/// service is allowed.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tac"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceAreaCoverageInfo {
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		/// Indicates a list of Tracking Areas where the service is allowed.
		#[serde(rename = "tacList")]
		pub tac_list: Vec<Tac>,
	}

	impl From<&ServiceAreaCoverageInfo> for ServiceAreaCoverageInfo {
		fn from(value: &ServiceAreaCoverageInfo) -> Self {
			value.clone()
		}
	}

	/// Identifies the service to which the subscription applies.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the service to which the subscription
	/// applies.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "not": {
	///        "required": [
	///          "servEthFlows",
	///          "servIpFlows"
	///        ]
	///      }
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "servEthFlows"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "servIpFlows"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "afAppId"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "servEthFlows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EthernetFlowInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "servIpFlows": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpFlowInfo"
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
	pub struct ServiceIdentification {
		#[serde(rename = "afAppId")]
		pub af_app_id: AfAppId,
	}

	impl From<&ServiceIdentification> for ServiceIdentification {
		fn from(value: &ServiceIdentification) -> Self {
			value.clone()
		}
	}

	/// Represents the preliminary or final service information status.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the preliminary or final service information
	/// status.",
	///  "type": "string",
	///  "enum": [
	///    "FINAL",
	///    "PRELIMINARY"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ServiceInfoStatus {
		#[default]
		#[serde(rename = "FINAL")]
		Final,
		#[serde(rename = "PRELIMINARY")]
		Preliminary,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ServiceInfoStatus> for ServiceInfoStatus {
		fn from(value: &ServiceInfoStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for ServiceInfoStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Final => "FINAL".to_string(),
				Self::Preliminary => "PRELIMINARY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ServiceInfoStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"FINAL" => Ok(Self::Final),
				"PRELIMINARY" => Ok(Self::Preliminary),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServiceInfoStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceInfoStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceInfoStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains values of the service URN and may include subservices.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains values of the service URN and may include
	/// subservices.",
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
	pub struct ServiceUrn(pub String);

	impl ::std::ops::Deref for ServiceUrn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ServiceUrn> for String {
		fn from(value: ServiceUrn) -> Self {
			value.0
		}
	}

	impl From<&ServiceUrn> for ServiceUrn {
		fn from(value: &ServiceUrn) -> Self {
			value.clone()
		}
	}

	impl From<String> for ServiceUrn {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ServiceUrn {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for ServiceUrn {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the serving Network Function identity.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the serving Network Function identity.",
	///  "type": "object",
	///  "properties": {
	///    "anGwAddr": {
	///      "$ref": "#/components/schemas/schemas-AnGwAddress"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "servNfInstId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "sgsnAddr": {
	///      "$ref": "#/components/schemas/SgsnAddress"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServingNfIdentity {
		#[serde(rename = "anGwAddr", default, skip_serializing_if = "Option::is_none")]
		pub an_gw_addr: Option<SchemasAnGwAddress>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "servNfInstId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serv_nf_inst_id: Option<NfInstanceId>,
		#[serde(rename = "sgsnAddr", default, skip_serializing_if = "Option::is_none")]
		pub sgsn_addr: Option<SgsnAddress>,
	}

	impl From<&ServingNfIdentity> for ServingNfIdentity {
		fn from(value: &ServingNfIdentity) -> Self {
			value.clone()
		}
	}

	/// Contains session level policy information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains session level policy information.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "sessRuleId"
	///  ],
	///  "properties": {
	///    "authDefQos": {
	///      "$ref": "#/components/schemas/AuthorizedDefaultQos"
	///    },
	///    "authSessAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "refCondData": {
	///      "description": "A reference to the condition data. It is the condId
	/// described in clause 5.6.2.9.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refUmData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type. It is the umId described in clause 5.6.2.12.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refUmN3gData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type to apply for Non-3GPP access. It is the umId described in clause
	/// 5.6.2.12.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "sessRuleId": {
	///      "description": "Univocally identifies the session rule within a PDU
	/// session.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SessionRule(pub Option<SessionRuleInner>);

	impl ::std::ops::Deref for SessionRule {
		type Target = Option<SessionRuleInner>;
		fn deref(&self) -> &Option<SessionRuleInner> {
			&self.0
		}
	}

	impl From<SessionRule> for Option<SessionRuleInner> {
		fn from(value: SessionRule) -> Self {
			value.0
		}
	}

	impl From<&SessionRule> for SessionRule {
		fn from(value: &SessionRule) -> Self {
			value.clone()
		}
	}

	impl From<Option<SessionRuleInner>> for SessionRule {
		fn from(value: Option<SessionRuleInner>) -> Self {
			Self(value)
		}
	}

	/// Possible values are
	/// - NF_MAL: Indicates that the PCC rule could not be successfully
	///   installed (for those provisioned from the PCF) or activated (for those
	///   pre-defined in SMF) or enforced (for those already successfully
	///   installed) due to SMF/UPF malfunction.
	/// - RES_LIM: Indicates that the PCC rule could not be successfully
	///   installed (for those provisioned from PCF) or activated (for those
	///   pre-defined in SMF) or enforced (for those already successfully
	///   installed) due to a limitation of resources at the SMF/UPF.
	/// - SESSION_RESOURCE_ALLOCATION_FAILURE: Indicates the session rule could
	///   not be successfully enforced due to failure during the allocation of
	///   resources for the PDU session in the UE, RAN or AMF.
	/// - UNSUCC_QOS_VAL: indicates that the QoS validation has failed.
	/// - INCORRECT_UM: The usage monitoring data of the enforced session rule
	///   is not the same for all the provisioned session rule(s).
	/// - UE_STA_SUSP: Indicates that the UE is in suspend state.
	/// - UNKNOWN_REF_ID: Indicates that the session rule could not be
	///   successfully installed/modified because the referenced identifier to a
	///   Policy Decision Data or to a Condition Data is unknown to the SMF.
	/// - INCORRECT_COND_DATA: Indicates that the session rule could not be
	///   successfully installed/modified because the referenced Condition data
	///   are incorrect.
	/// - REF_ID_COLLISION: Indicates that the session rule could not be
	///   successfully installed/modified because the same Policy Decision is
	///   referenced by a PCC rule (e.g. the session rule and the PCC rule refer
	///   to the same Usage Monitoring decision data).
	/// - AN_GW_FAILED: Indicates that the AN-Gateway has failed and that the
	///   PCF should refrain from sending policy decisions to the SMF until it
	///   is informed that the S-GW has been recovered. This value shall not be
	///   used if the SM Policy association modification procedure is initiated
	///   for session rule removal only.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- NF_MAL: Indicates that the PCC
	/// rule could not be successfully installed (for those provisioned from the
	/// PCF) or activated (for those pre-defined in SMF) or enforced (for those
	/// already successfully installed) due to SMF/UPF malfunction.\n- RES_LIM:
	/// Indicates that the PCC rule could not be successfully installed (for
	/// those provisioned from PCF) or activated (for those pre-defined in SMF)
	/// or enforced (for those already successfully installed) due to a
	/// limitation of resources at the SMF/UPF.\n-
	/// SESSION_RESOURCE_ALLOCATION_FAILURE: Indicates the session rule could
	/// not be successfully enforced due to failure during the allocation of
	/// resources for the PDU session in the UE, RAN or AMF.\n- UNSUCC_QOS_VAL:
	/// indicates that the QoS validation has failed.\n- INCORRECT_UM: The usage
	/// monitoring data of the enforced session rule is not the same for all the
	/// provisioned session rule(s).\n- UE_STA_SUSP: Indicates that the UE is in
	/// suspend state.\n- UNKNOWN_REF_ID: Indicates that the session rule could
	/// not be successfully installed/modified because the referenced identifier
	/// to a Policy Decision Data or to a Condition Data is unknown to the
	/// SMF.\n- INCORRECT_COND_DATA: Indicates that the session rule could not
	/// be successfully installed/modified because the referenced Condition data
	/// are incorrect.\n- REF_ID_COLLISION: Indicates that the session rule
	/// could not be successfully installed/modified because the same Policy
	/// Decision is referenced by a PCC rule (e.g. the session rule and the PCC
	/// rule refer to the same Usage Monitoring decision data).\n- AN_GW_FAILED:
	/// Indicates that the AN-Gateway has failed and that the PCF should refrain
	/// from sending policy decisions to the SMF until it is informed that the
	/// S-GW has been recovered. This value shall not be used if the SM Policy
	/// association modification procedure is initiated for session rule removal
	/// only.\n",
	///  "type": "string",
	///  "enum": [
	///    "NF_MAL",
	///    "RES_LIM",
	///    "SESSION_RESOURCE_ALLOCATION_FAILURE",
	///    "UNSUCC_QOS_VAL",
	///    "INCORRECT_UM",
	///    "UE_STA_SUSP",
	///    "UNKNOWN_REF_ID",
	///    "INCORRECT_COND_DATA",
	///    "REF_ID_COLLISION",
	///    "AN_GW_FAILED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SessionRuleFailureCode {
		#[default]
		#[serde(rename = "NF_MAL")]
		NfMal,
		#[serde(rename = "RES_LIM")]
		ResLim,
		#[serde(rename = "SESSION_RESOURCE_ALLOCATION_FAILURE")]
		SessionResourceAllocationFailure,
		#[serde(rename = "UNSUCC_QOS_VAL")]
		UnsuccQosVal,
		#[serde(rename = "INCORRECT_UM")]
		IncorrectUm,
		#[serde(rename = "UE_STA_SUSP")]
		UeStaSusp,
		#[serde(rename = "UNKNOWN_REF_ID")]
		UnknownRefId,
		#[serde(rename = "INCORRECT_COND_DATA")]
		IncorrectCondData,
		#[serde(rename = "REF_ID_COLLISION")]
		RefIdCollision,
		#[serde(rename = "AN_GW_FAILED")]
		AnGwFailed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SessionRuleFailureCode> for SessionRuleFailureCode {
		fn from(value: &SessionRuleFailureCode) -> Self {
			value.clone()
		}
	}

	impl ToString for SessionRuleFailureCode {
		fn to_string(&self) -> String {
			match *self {
				Self::NfMal => "NF_MAL".to_string(),
				Self::ResLim => "RES_LIM".to_string(),
				Self::SessionResourceAllocationFailure => {
					"SESSION_RESOURCE_ALLOCATION_FAILURE".to_string()
				}
				Self::UnsuccQosVal => "UNSUCC_QOS_VAL".to_string(),
				Self::IncorrectUm => "INCORRECT_UM".to_string(),
				Self::UeStaSusp => "UE_STA_SUSP".to_string(),
				Self::UnknownRefId => "UNKNOWN_REF_ID".to_string(),
				Self::IncorrectCondData => "INCORRECT_COND_DATA".to_string(),
				Self::RefIdCollision => "REF_ID_COLLISION".to_string(),
				Self::AnGwFailed => "AN_GW_FAILED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SessionRuleFailureCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_MAL" => Ok(Self::NfMal),
				"RES_LIM" => Ok(Self::ResLim),
				"SESSION_RESOURCE_ALLOCATION_FAILURE" => Ok(Self::SessionResourceAllocationFailure),
				"UNSUCC_QOS_VAL" => Ok(Self::UnsuccQosVal),
				"INCORRECT_UM" => Ok(Self::IncorrectUm),
				"UE_STA_SUSP" => Ok(Self::UeStaSusp),
				"UNKNOWN_REF_ID" => Ok(Self::UnknownRefId),
				"INCORRECT_COND_DATA" => Ok(Self::IncorrectCondData),
				"REF_ID_COLLISION" => Ok(Self::RefIdCollision),
				"AN_GW_FAILED" => Ok(Self::AnGwFailed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SessionRuleFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SessionRuleFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SessionRuleFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains session level policy information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains session level policy information.",
	///  "type": "object",
	///  "required": [
	///    "sessRuleId"
	///  ],
	///  "properties": {
	///    "authDefQos": {
	///      "$ref": "#/components/schemas/AuthorizedDefaultQos"
	///    },
	///    "authSessAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "refCondData": {
	///      "description": "A reference to the condition data. It is the condId
	/// described in clause 5.6.2.9.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refUmData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type. It is the umId described in clause 5.6.2.12.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "refUmN3gData": {
	///      "description": "A reference to UsageMonitoringData policy decision
	/// type to apply for Non-3GPP access. It is the umId described in clause
	/// 5.6.2.12.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "sessRuleId": {
	///      "description": "Univocally identifies the session rule within a PDU
	/// session.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SessionRuleInner {
		#[serde(
			rename = "authDefQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_def_qos: Option<AuthorizedDefaultQos>,
		#[serde(
			rename = "authSessAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_sess_ambr: Option<Ambr>,
		/// A reference to the condition data. It is the condId described in
		/// clause 5.6.2.9.
		#[serde(
			rename = "refCondData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_cond_data: Option<String>,
		/// A reference to UsageMonitoringData policy decision type. It is the
		/// umId described in clause 5.6.2.12.
		#[serde(rename = "refUmData", default, skip_serializing_if = "Option::is_none")]
		pub ref_um_data: Option<String>,
		/// A reference to UsageMonitoringData policy decision type to apply for
		/// Non-3GPP access. It is the umId described in clause 5.6.2.12.
		#[serde(
			rename = "refUmN3gData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_um_n3g_data: Option<String>,
		/// Univocally identifies the session rule within a PDU session.
		#[serde(rename = "sessRuleId")]
		pub sess_rule_id: String,
	}

	impl From<&SessionRuleInner> for SessionRuleInner {
		fn from(value: &SessionRuleInner) -> Self {
			value.clone()
		}
	}

	/// Represents reporting of the status of a session rule.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents reporting of the status of a session rule.",
	///  "type": "object",
	///  "required": [
	///    "ruleIds",
	///    "ruleStatus"
	///  ],
	///  "properties": {
	///    "policyDecFailureReports": {
	///      "description": "Contains the type(s) of failed policy decision
	/// and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyDecisionFailureCode"
	///      },
	///      "minItems": 1
	///    },
	///    "ruleIds": {
	///      "description": "Contains the identifier of the affected session
	/// rule(s).",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ruleStatus": {
	///      "$ref": "#/components/schemas/RuleStatus"
	///    },
	///    "sessRuleFailureCode": {
	///      "$ref": "#/components/schemas/SessionRuleFailureCode"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SessionRuleReport {
		/// Contains the type(s) of failed policy decision and/or condition
		/// data.
		#[serde(
			rename = "policyDecFailureReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub policy_dec_failure_reports: Vec<PolicyDecisionFailureCode>,
		/// Contains the identifier of the affected session rule(s).
		#[serde(rename = "ruleIds")]
		pub rule_ids: Vec<String>,
		#[serde(rename = "ruleStatus")]
		pub rule_status: RuleStatus,
		#[serde(
			rename = "sessRuleFailureCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sess_rule_failure_code: Option<SessionRuleFailureCode>,
	}

	impl From<&SessionRuleReport> for SessionRuleReport {
		fn from(value: &SessionRuleReport) -> Self {
			value.clone()
		}
	}

	/// describes the address of the SGSN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "describes the address of the SGSN",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "sgsnIpv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "sgsnIpv6Addr"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "sgsnIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "sgsnIpv6Addr": {
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
	pub enum SgsnAddress {
		#[default]
		Variant0 {
			#[serde(rename = "sgsnIpv4Addr")]
			sgsn_ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "sgsnIpv6Addr")]
			sgsn_ipv6_addr: Ipv6Addr,
		},
	}

	impl From<&SgsnAddress> for SgsnAddress {
		fn from(value: &SgsnAddress) -> Self {
			value.clone()
		}
	}

	/// Indicates whether several SIP dialogues are related to an "Individual
	/// Application Session Context" resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether several SIP dialogues are related to
	/// an \"Individual Application Session Context\" resource.",
	///  "type": "string",
	///  "enum": [
	///    "SINGLE_DIALOGUE",
	///    "SEVERAL_DIALOGUES"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SipForkingIndication {
		#[default]
		#[serde(rename = "SINGLE_DIALOGUE")]
		SingleDialogue,
		#[serde(rename = "SEVERAL_DIALOGUES")]
		SeveralDialogues,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SipForkingIndication> for SipForkingIndication {
		fn from(value: &SipForkingIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for SipForkingIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::SingleDialogue => "SINGLE_DIALOGUE".to_string(),
				Self::SeveralDialogues => "SEVERAL_DIALOGUES".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SipForkingIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SINGLE_DIALOGUE" => Ok(Self::SingleDialogue),
				"SEVERAL_DIALOGUES" => Ok(Self::SeveralDialogues),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SipForkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SipForkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SipForkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the cause due to which the PCF requests the termination of
	/// the SM policy association.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the cause due to which the PCF requests the
	/// termination of the SM policy association.",
	///  "type": "string",
	///  "enum": [
	///    "UNSPECIFIED",
	///    "UE_SUBSCRIPTION",
	///    "INSUFFICIENT_RES",
	///    "VALIDATION_CONDITION_NOT_MET",
	///    "REACTIVATION_REQUESTED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SmPolicyAssociationReleaseCause {
		#[default]
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(rename = "UE_SUBSCRIPTION")]
		UeSubscription,
		#[serde(rename = "INSUFFICIENT_RES")]
		InsufficientRes,
		#[serde(rename = "VALIDATION_CONDITION_NOT_MET")]
		ValidationConditionNotMet,
		#[serde(rename = "REACTIVATION_REQUESTED")]
		ReactivationRequested,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmPolicyAssociationReleaseCause> for SmPolicyAssociationReleaseCause {
		fn from(value: &SmPolicyAssociationReleaseCause) -> Self {
			value.clone()
		}
	}

	impl ToString for SmPolicyAssociationReleaseCause {
		fn to_string(&self) -> String {
			match *self {
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UeSubscription => "UE_SUBSCRIPTION".to_string(),
				Self::InsufficientRes => "INSUFFICIENT_RES".to_string(),
				Self::ValidationConditionNotMet => "VALIDATION_CONDITION_NOT_MET".to_string(),
				Self::ReactivationRequested => "REACTIVATION_REQUESTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmPolicyAssociationReleaseCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNSPECIFIED" => Ok(Self::Unspecified),
				"UE_SUBSCRIPTION" => Ok(Self::UeSubscription),
				"INSUFFICIENT_RES" => Ok(Self::InsufficientRes),
				"VALIDATION_CONDITION_NOT_MET" => Ok(Self::ValidationConditionNotMet),
				"REACTIVATION_REQUESTED" => Ok(Self::ReactivationRequested),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmPolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmPolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmPolicyAssociationReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the parameters used to create an Individual SM policy resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the parameters used to create an Individual SM
	/// policy resource.",
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "notificationUri",
	///    "pduSessionId",
	///    "pduSessionType",
	///    "sliceInfo",
	///    "supi"
	///  ],
	///  "properties": {
	///    "3gppPsDataOffStatus": {
	///      "description": "If it is included and set to true, the 3GPP PS Data
	/// Off is activated by the UE.",
	///      "type": "boolean"
	///    },
	///    "accNetChId": {
	///      "$ref": "#/components/schemas/AccNetChId"
	///    },
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "addAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "atsssCapab": {
	///      "$ref": "#/components/schemas/AtsssCapability"
	///    },
	///    "authProfIndex": {
	///      "description": "Indicates the DN-AAA authorization profile index",
	///      "type": "string"
	///    },
	///    "chargEntityAddr": {
	///      "$ref": "#/components/schemas/AccNetChargingAddress"
	///    },
	///    "chargingcharacteristics": {
	///      "type": "string"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "dnnSelMode": {
	///      "$ref": "#/components/schemas/DnnSelectionMode"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "interGrpIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "invalidSupi": {
	///      "description": "When this attribute is included and set to true, it
	/// indicates that the supi attribute contains an invalid value.This
	/// attribute shall be present if the SUPI is not available in the SMF or
	/// the SUPI is unauthenticated. When present it shall be set to true for an
	/// invalid SUPI and false (default) for a valid SUPI.\n",
	///      "type": "boolean"
	///    },
	///    "ipDomain": {
	///      "description": "Indicates the IPv4 address domain",
	///      "type": "string"
	///    },
	///    "ipv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv4FrameRouteList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4AddrMask"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6AddressPrefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "ipv6FrameRouteList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Prefix"
	///      },
	///      "minItems": 1
	///    },
	///    "maPduInd": {
	///      "$ref": "#/components/schemas/MaPduIndication"
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "numOfPackFilter": {
	///      "description": "Contains the number of supported packet filter for
	/// signalled QoS rules.",
	///      "type": "integer"
	///    },
	///    "nwdafDatas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafData"
	///      },
	///      "minItems": 1
	///    },
	///    "offline": {
	///      "description": "If it is included and set to true, the offline
	/// charging is applied to the PDU session.",
	///      "type": "boolean"
	///    },
	///    "onboardInd": {
	///      "description": "If it is included and set to true, it indicates
	/// that the PDU session is used for UE Onboarding.",
	///      "type": "boolean"
	///    },
	///    "online": {
	///      "description": "If it is included and set to true, the online
	/// charging is applied to the PDU session.",
	///      "type": "boolean"
	///    },
	///    "pcfUeInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessionType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "pvsInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServerAddressingInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowUsage": {
	///      "$ref": "#/components/schemas/QosFlowUsage"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "refQosIndication": {
	///      "description": "If it is included and set to true, the reflective
	/// QoS is supported by the UE.",
	///      "type": "boolean"
	///    },
	///    "satBackhaulCategory": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "servNfId": {
	///      "$ref": "#/components/schemas/ServingNfIdentity"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "sliceInfo": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "smfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "subsDefQos": {
	///      "$ref": "#/components/schemas/SubscribedDefaultQos"
	///    },
	///    "subsSessAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traceReq": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "vplmnQos": {
	///      "$ref": "#/components/schemas/VplmnQos"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyContextData {
		#[serde(
			rename = "accNetChId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub acc_net_ch_id: Option<AccNetChId>,
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(
			rename = "addAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "atsssCapab",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub atsss_capab: Option<AtsssCapability>,
		/// Indicates the DN-AAA authorization profile index
		#[serde(
			rename = "authProfIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_prof_index: Option<String>,
		#[serde(
			rename = "chargEntityAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charg_entity_addr: Option<AccNetChargingAddress>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub chargingcharacteristics: Option<String>,
		pub dnn: Dnn,
		#[serde(
			rename = "dnnSelMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dnn_sel_mode: Option<DnnSelectionMode>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "interGrpIds", default, skip_serializing_if = "Vec::is_empty")]
		pub inter_grp_ids: Vec<GroupId>,
		/// When this attribute is included and set to true, it indicates that
		/// the supi attribute contains an invalid value.This attribute shall be
		/// present if the SUPI is not available in the SMF or the SUPI is
		/// unauthenticated. When present it shall be set to true for an invalid
		/// SUPI and false (default) for a valid SUPI.
		#[serde(
			rename = "invalidSupi",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invalid_supi: Option<bool>,
		/// Indicates the IPv4 address domain
		#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
		pub ip_domain: Option<String>,
		#[serde(
			rename = "ipv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "ipv4FrameRouteList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_frame_route_list: Vec<Ipv4AddrMask>,
		#[serde(
			rename = "ipv6AddressPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_address_prefix: Option<Ipv6Prefix>,
		#[serde(
			rename = "ipv6FrameRouteList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_frame_route_list: Vec<Ipv6Prefix>,
		#[serde(rename = "maPduInd", default, skip_serializing_if = "Option::is_none")]
		pub ma_pdu_ind: Option<MaPduIndication>,
		#[serde(rename = "notificationUri")]
		pub notification_uri: Uri,
		/// Contains the number of supported packet filter for signalled QoS
		/// rules.
		#[serde(
			rename = "numOfPackFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_of_pack_filter: Option<i64>,
		#[serde(rename = "nwdafDatas", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_datas: Vec<NwdafData>,
		/// If it is included and set to true, the offline charging is applied
		/// to the PDU session.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub offline: Option<bool>,
		/// If it is included and set to true, it indicates that the PDU session
		/// is used for UE Onboarding.
		#[serde(
			rename = "onboardInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub onboard_ind: Option<bool>,
		/// If it is included and set to true, the online charging is applied to
		/// the PDU session.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub online: Option<bool>,
		#[serde(rename = "pcfUeInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_ue_info: Option<PcfUeCallbackInfo>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(rename = "pduSessionType")]
		pub pdu_session_type: PduSessionType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "pvsInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pvs_info: Vec<ServerAddressingInfo>,
		#[serde(
			rename = "qosFlowUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_usage: Option<QosFlowUsage>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		/// If it is included and set to true, the reflective QoS is supported
		/// by the UE.
		#[serde(
			rename = "refQosIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_qos_indication: Option<bool>,
		#[serde(
			rename = "satBackhaulCategory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sat_backhaul_category: Option<SatelliteBackhaulCategory>,
		#[serde(rename = "servNfId", default, skip_serializing_if = "Option::is_none")]
		pub serv_nf_id: Option<ServingNfIdentity>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(rename = "sliceInfo")]
		pub slice_info: Snssai,
		#[serde(rename = "smfId", default, skip_serializing_if = "Option::is_none")]
		pub smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "subsDefQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_def_qos: Option<SubscribedDefaultQos>,
		#[serde(
			rename = "subsSessAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_sess_ambr: Option<Ambr>,
		pub supi: Supi,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
		/// If it is included and set to true, the 3GPP PS Data Off is activated
		/// by the UE.
		#[serde(
			rename = "3gppPsDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<bool>,
		#[serde(rename = "traceReq", default, skip_serializing_if = "Option::is_none")]
		pub trace_req: Option<TraceData>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info: Option<UserLocation>,
		#[serde(rename = "vplmnQos", default, skip_serializing_if = "Option::is_none")]
		pub vplmn_qos: Option<VplmnQos>,
	}

	impl From<&SmPolicyContextData> for SmPolicyContextData {
		fn from(value: &SmPolicyContextData) -> Self {
			value.clone()
		}
	}

	/// Contains the parameters used to request the SM policies and the SM
	/// policies authorized by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the parameters used to request the SM policies
	/// and the SM policies authorized by the PCF.",
	///  "type": "object",
	///  "required": [
	///    "context",
	///    "policy"
	///  ],
	///  "properties": {
	///    "context": {
	///      "$ref": "#/components/schemas/SmPolicyContextData"
	///    },
	///    "policy": {
	///      "$ref": "#/components/schemas/SmPolicyDecision"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyControl {
		pub context: SmPolicyContextData,
		pub policy: SmPolicyDecision,
	}

	impl From<&SmPolicyControl> for SmPolicyControl {
		fn from(value: &SmPolicyControl) -> Self {
			value.clone()
		}
	}

	/// Contains the SM policies authorized by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the SM policies authorized by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "chargingInfo": {
	///      "$ref": "#/components/schemas/ChargingInformation"
	///    },
	///    "chgDecs": {
	///      "description": "Map of Charging data policy decisions. The key used
	/// in this map for each entry is the chgId attribute of the corresponding
	/// ChargingData.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/ChargingData"
	///      }
	///    },
	///    "conds": {
	///      "description": "A map of condition data with the content being as
	/// described in clause 5.6.2.9. The key used in this map for each entry is
	/// the condId attribute of the corresponding ConditionData.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/ConditionData"
	///      }
	///    },
	///    "ipv4Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "ipv6Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "lastReqRuleData": {
	///      "description": "Defines the last list of rule control data
	/// requested by the PCF.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RequestedRuleData"
	///      },
	///      "minItems": 1
	///    },
	///    "lastReqUsageData": {
	///      "$ref": "#/components/schemas/RequestedUsageData"
	///    },
	///    "offline": {
	///      "description": "Indicates the offline charging is applicable to the
	/// PDU session when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "offlineChOnly": {
	///      "description": "Indicates that the online charging method shall
	/// never be used for any PCC rule activated during the lifetime of the PDU
	/// session.\n",
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "online": {
	///      "description": "Indicates the online charging is applicable to the
	/// PDU session when it is included and set to true.",
	///      "type": "boolean"
	///    },
	///    "pccRules": {
	///      "description": "A map of PCC rules with the content being the PCCRule as described in  clause 5.6.2.6. The key used in this map for each entry is the pccRuleId attribute of the corresponding PccRule.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PccRule"
	///      }
	///    },
	///    "pcscfRestIndication": {
	///      "description": "If it is included and set to true, it indicates the
	/// P-CSCF Restoration is requested.",
	///      "type": "boolean"
	///    },
	///    "policyCtrlReqTriggers": {
	///      "description": "Defines the policy control request triggers
	/// subscribed by the PCF.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyControlRequestTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "praInfos": {
	///      "description": "Map of PRA information. The praId attribute within
	/// the PresenceInfo data type is the key of the map.",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfoRm"
	///      }
	///    },
	///    "qosChars": {
	///      "description": "Map of QoS characteristics for non standard 5QIs.
	/// This map uses the 5QI values as keys.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/QosCharacteristics"
	///      }
	///    },
	///    "qosDecs": {
	///      "description": "Map of QoS data policy decisions. The key used in
	/// this map for each entry is the qosId attribute of the corresponding
	/// QosData.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/QosData"
	///      }
	///    },
	///    "qosFlowUsage": {
	///      "$ref": "#/components/schemas/QosFlowUsage"
	///    },
	///    "qosMonDecs": {
	///      "description": "Map of QoS Monitoring data policy decisions. The
	/// key used in this map for each entry is the qmId attribute of the
	/// corresponding QosMonitoringData.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/QosMonitoringData"
	///      }
	///    },
	///    "redSessIndication": {
	///      "description": "Indicates whether the PDU session is a redundant
	/// PDU session. If absent it means the PDU session is not a redundant PDU
	/// session.\n",
	///      "type": "boolean"
	///    },
	///    "reflectiveQoSTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "relCause": {
	///      "$ref": "#/components/schemas/SmPolicyAssociationReleaseCause"
	///    },
	///    "revalidationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sessRules": {
	///      "description": "A map of Sessionrules with the content being the
	/// SessionRule as described in clause 5.6.2.7. The key used in this map for
	/// each entry is the sessRuleId attribute of the corresponding
	/// SessionRule.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SessionRule"
	///      }
	///    },
	///    "suppFeat": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traffContDecs": {
	///      "description": "Map of Traffic Control data policy decisions. The
	/// key used in this map for each entry is the tcId attribute of the
	/// corresponding TrafficControlData.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/TrafficControlData"
	///      }
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "umDecs": {
	///      "description": "Map of Usage Monitoring data policy decisions. The
	/// key used in this map for each entry is the umId attribute of the
	/// corresponding UsageMonitoringData.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/UsageMonitoringData"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyDecision {
		#[serde(
			rename = "chargingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_info: Option<ChargingInformation>,
		/// Map of Charging data policy decisions. The key used in this map for
		/// each entry is the chgId attribute of the corresponding ChargingData.
		#[serde(rename = "chgDecs", default, skip_serializing_if = "Option::is_none")]
		pub chg_decs: Option<::std::collections::HashMap<String, ChargingData>>,
		/// A map of condition data with the content being as described in
		/// clause 5.6.2.9. The key used in this map for each entry is the
		/// condId attribute of the corresponding ConditionData.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub conds: Option<::std::collections::HashMap<String, ConditionData>>,
		#[serde(rename = "ipv4Index", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_index: Option<IpIndex>,
		#[serde(rename = "ipv6Index", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_index: Option<IpIndex>,
		/// Defines the last list of rule control data requested by the PCF.
		#[serde(
			rename = "lastReqRuleData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub last_req_rule_data: Vec<RequestedRuleData>,
		#[serde(
			rename = "lastReqUsageData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_req_usage_data: Option<RequestedUsageData>,
		/// Indicates the offline charging is applicable to the PDU session when
		/// it is included and set to true.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub offline: Option<bool>,
		/// Indicates that the online charging method shall never be used for
		/// any PCC rule activated during the lifetime of the PDU session.
		#[serde(rename = "offlineChOnly", default)]
		pub offline_ch_only: bool,
		/// Indicates the online charging is applicable to the PDU session when
		/// it is included and set to true.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub online: Option<bool>,
		/// A map of PCC rules with the content being the PCCRule as described
		/// in  clause 5.6.2.6. The key used in this map for each entry is the
		/// pccRuleId attribute of the corresponding PccRule.
		#[serde(rename = "pccRules", default, skip_serializing_if = "Option::is_none")]
		pub pcc_rules: Option<::std::collections::HashMap<String, PccRule>>,
		/// If it is included and set to true, it indicates the P-CSCF
		/// Restoration is requested.
		#[serde(
			rename = "pcscfRestIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcscf_rest_indication: Option<bool>,
		/// Defines the policy control request triggers subscribed by the PCF.
		#[serde(
			rename = "policyCtrlReqTriggers",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub policy_ctrl_req_triggers: Option<Vec<PolicyControlRequestTrigger>>,
		/// Map of PRA information. The praId attribute within the PresenceInfo
		/// data type is the key of the map.
		#[serde(rename = "praInfos", default, skip_serializing_if = "Option::is_none")]
		pub pra_infos: Option<::std::collections::HashMap<String, PresenceInfoRm>>,
		/// Map of QoS characteristics for non standard 5QIs. This map uses the
		/// 5QI values as keys.
		#[serde(
			rename = "qosChars",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub qos_chars: ::std::collections::HashMap<String, QosCharacteristics>,
		/// Map of QoS data policy decisions. The key used in this map for each
		/// entry is the qosId attribute of the corresponding QosData.
		#[serde(
			rename = "qosDecs",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub qos_decs: ::std::collections::HashMap<String, QosData>,
		#[serde(
			rename = "qosFlowUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_usage: Option<QosFlowUsage>,
		/// Map of QoS Monitoring data policy decisions. The key used in this
		/// map for each entry is the qmId attribute of the corresponding
		/// QosMonitoringData.
		#[serde(
			rename = "qosMonDecs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_mon_decs: Option<::std::collections::HashMap<String, QosMonitoringData>>,
		/// Indicates whether the PDU session is a redundant PDU session. If
		/// absent it means the PDU session is not a redundant PDU session.
		#[serde(
			rename = "redSessIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub red_sess_indication: Option<bool>,
		#[serde(
			rename = "reflectiveQoSTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reflective_qo_s_timer: Option<DurationSec>,
		#[serde(rename = "relCause", default, skip_serializing_if = "Option::is_none")]
		pub rel_cause: Option<SmPolicyAssociationReleaseCause>,
		#[serde(
			rename = "revalidationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub revalidation_time: Option<DateTime>,
		/// A map of Sessionrules with the content being the SessionRule as
		/// described in clause 5.6.2.7. The key used in this map for each entry
		/// is the sessRuleId attribute of the corresponding SessionRule.
		#[serde(
			rename = "sessRules",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub sess_rules: ::std::collections::HashMap<String, SessionRule>,
		#[serde(rename = "suppFeat", default, skip_serializing_if = "Option::is_none")]
		pub supp_feat: Option<SupportedFeatures>,
		/// Map of Traffic Control data policy decisions. The key used in this
		/// map for each entry is the tcId attribute of the corresponding
		/// TrafficControlData.
		#[serde(
			rename = "traffContDecs",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub traff_cont_decs: ::std::collections::HashMap<String, TrafficControlData>,
		#[serde(
			rename = "tsnBridgeManCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_man_cont: Option<BridgeManagementContainer>,
		#[serde(
			rename = "tsnPortManContDstt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_port_man_cont_dstt: Option<PortManagementContainer>,
		#[serde(
			rename = "tsnPortManContNwtts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
		/// Map of Usage Monitoring data policy decisions. The key used in this
		/// map for each entry is the umId attribute of the corresponding
		/// UsageMonitoringData.
		#[serde(rename = "umDecs", default, skip_serializing_if = "Option::is_none")]
		pub um_decs: Option<::std::collections::HashMap<String, UsageMonitoringData>>,
	}

	impl From<&SmPolicyDecision> for SmPolicyDecision {
		fn from(value: &SmPolicyDecision) -> Self {
			value.clone()
		}
	}

	/// Contains the parameters to be sent to the PCF when an individual SM
	/// policy is deleted.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the parameters to be sent to the PCF when an
	/// individual SM policy is deleted.",
	///  "type": "object",
	///  "properties": {
	///    "accuUsageReports": {
	///      "description": "Contains the usage report",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccuUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessRelCause": {
	///      "$ref": "#/components/schemas/PduSessionRelCause"
	///    },
	///    "ranNasRelCauses": {
	///      "description": "Contains the RAN and/or NAS release cause.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RanNasRelCause"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "userLocationInfoTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyDeleteData {
		/// Contains the usage report
		#[serde(
			rename = "accuUsageReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub accu_usage_reports: Vec<AccuUsageReport>,
		#[serde(
			rename = "pduSessRelCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_sess_rel_cause: Option<PduSessionRelCause>,
		/// Contains the RAN and/or NAS release cause.
		#[serde(
			rename = "ranNasRelCauses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ran_nas_rel_causes: Vec<RanNasRelCause>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info: Option<UserLocation>,
		#[serde(
			rename = "userLocationInfoTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info_time: Option<DateTime>,
	}

	impl From<&SmPolicyDeleteData> for SmPolicyDeleteData {
		fn from(value: &SmPolicyDeleteData) -> Self {
			value.clone()
		}
	}

	/// Represents a notification on the update of the SM policies.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a notification on the update of the SM
	/// policies.",
	///  "type": "object",
	///  "properties": {
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smPolicyDecision": {
	///      "$ref": "#/components/schemas/SmPolicyDecision"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyNotification {
		#[serde(
			rename = "resourceUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resource_uri: Option<Uri>,
		#[serde(
			rename = "smPolicyDecision",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_policy_decision: Option<SmPolicyDecision>,
	}

	impl From<&SmPolicyNotification> for SmPolicyNotification {
		fn from(value: &SmPolicyNotification) -> Self {
			value.clone()
		}
	}

	/// Contains the policy control request trigger(s) that were met and the
	/// corresponding new value(s) or the error report of the policy
	/// enforcement.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the policy control request trigger(s) that
	/// were met and the corresponding new value(s) or the error report of the
	/// policy enforcement.",
	///  "type": "object",
	///  "properties": {
	///    "3gppPsDataOffStatus": {
	///      "description": "If it is included and set to true, the 3GPP PS Data
	/// Off is activated by the UE.",
	///      "type": "boolean"
	///    },
	///    "accNetChIds": {
	///      "description": "Indicates the access network charging identifier
	/// for the PCC rule(s) or whole PDU session.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccNetChId"
	///      },
	///      "minItems": 1
	///    },
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "accuUsageReports": {
	///      "description": "Contains the usage report",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccuUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "addAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "addIpv6AddrPrefixes": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "addRelIpv6AddrPrefixes": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "anGwStatus": {
	///      "description": "When it is included and set to true, it indicates
	/// that the AN-Gateway has failed and that the PCF should refrain from
	/// sending policy decisions to the SMF until it is informed that the
	/// AN-Gateway has been recovered.\n",
	///      "type": "boolean"
	///    },
	///    "appDetectionInfos": {
	///      "description": "Report the start/stop of the application traffic
	/// and detected SDF descriptions if applicable.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AppDetectionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "atsssCapab": {
	///      "$ref": "#/components/schemas/AtsssCapability"
	///    },
	///    "authProfIndex": {
	///      "description": "Indicates the DN-AAA authorization profile index",
	///      "type": "string"
	///    },
	///    "creditManageStatus": {
	///      "$ref": "#/components/schemas/CreditManagementStatus"
	///    },
	///    "interGrpIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "invalidPolicyDecs": {
	///      "description": "Indicates the invalid parameters for the reported
	/// type(s) of the failed policy decision and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InvalidParam"
	///      },
	///      "minItems": 1
	///    },
	///    "ipDomain": {
	///      "description": "Indicates the IPv4 address domain",
	///      "type": "string"
	///    },
	///    "ipv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6AddressPrefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "maPduInd": {
	///      "$ref": "#/components/schemas/MaPduIndication"
	///    },
	///    "mulAddrInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpMulticastAddressInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "numOfPackFilter": {
	///      "description": "Contains the number of supported packet filter for
	/// signalled QoS rules.",
	///      "type": "integer"
	///    },
	///    "nwdafDatas": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafData"
	///      },
	///      "minItems": 1
	///    },
	///    "pccRuleId": {
	///      "description": "Contains the identifier of the PCC rule which is
	/// used for traffic detection of event.",
	///      "type": "string"
	///    },
	///    "pcfUeInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "policyDecFailureReports": {
	///      "description": "Contains the type(s) of failed policy decision
	/// and/or condition data.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyDecisionFailureCode"
	///      },
	///      "minItems": 1
	///    },
	///    "qncReports": {
	///      "description": "QoS Notification Control information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosNotificationControlInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowUsage": {
	///      "$ref": "#/components/schemas/QosFlowUsage"
	///    },
	///    "qosMonReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosMonitoringReport"
	///      },
	///      "minItems": 1
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "refQosIndication": {
	///      "description": "If it is included and set to true, the reflective
	/// QoS is supported by the UE. If it is included and set to false, the
	/// reflective QoS is revoked by the UE.\n",
	///      "type": "boolean"
	///    },
	///    "relAccessInfo": {
	///      "$ref": "#/components/schemas/AdditionalAccessInfo"
	///    },
	///    "relIpv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "relIpv6AddressPrefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "relUeMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "repPolicyCtrlReqTriggers": {
	///      "description": "The policy control reqeust trigges which are met.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyControlRequestTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "repPraInfos": {
	///      "description": "Reports the changes of presence reporting area. The
	/// praId attribute within the PresenceInfo data type is the key of the
	/// map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "ruleReports": {
	///      "description": "Used to report the PCC rule failure.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RuleReport"
	///      },
	///      "minItems": 1
	///    },
	///    "satBackhaulCategory": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "servNfId": {
	///      "$ref": "#/components/schemas/ServingNfIdentity"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "sessRuleReports": {
	///      "description": "Used to report the session rule failure.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SessionRuleReport"
	///      },
	///      "minItems": 1
	///    },
	///    "subsDefQos": {
	///      "$ref": "#/components/schemas/SubscribedDefaultQos"
	///    },
	///    "subsSessAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "traceReq": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "trafficDescriptors": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "tsnBridgeInfo": {
	///      "$ref": "#/components/schemas/TsnBridgeInfo"
	///    },
	///    "tsnBridgeManCont": {
	///      "$ref": "#/components/schemas/BridgeManagementContainer"
	///    },
	///    "tsnPortManContDstt": {
	///      "$ref": "#/components/schemas/PortManagementContainer"
	///    },
	///    "tsnPortManContNwtts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PortManagementContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "typesOfNotif": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
	///      },
	///      "minItems": 1
	///    },
	///    "ueInitResReq": {
	///      "$ref": "#/components/schemas/UeInitiatedResourceRequest"
	///    },
	///    "ueMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "userLocationInfoTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "vplmnQos": {
	///      "$ref": "#/components/schemas/VplmnQos"
	///    },
	///    "vplmnQosNotApp": {
	///      "description": "If it is included and set to true, indicates that
	/// the QoS constraints in the VPLMN are not applicable.",
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmPolicyUpdateContextData {
		/// Indicates the access network charging identifier for the PCC rule(s)
		/// or whole PDU session.
		#[serde(rename = "accNetChIds", default, skip_serializing_if = "Vec::is_empty")]
		pub acc_net_ch_ids: Vec<AccNetChId>,
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		/// Contains the usage report
		#[serde(
			rename = "accuUsageReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub accu_usage_reports: Vec<AccuUsageReport>,
		#[serde(
			rename = "addAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "addIpv6AddrPrefixes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ipv6_addr_prefixes: Option<Ipv6Prefix>,
		#[serde(
			rename = "addRelIpv6AddrPrefixes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_rel_ipv6_addr_prefixes: Option<Ipv6Prefix>,
		/// When it is included and set to true, it indicates that the
		/// AN-Gateway has failed and that the PCF should refrain from sending
		/// policy decisions to the SMF until it is informed that the AN-Gateway
		/// has been recovered.
		#[serde(
			rename = "anGwStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_gw_status: Option<bool>,
		/// Report the start/stop of the application traffic and detected SDF
		/// descriptions if applicable.
		#[serde(
			rename = "appDetectionInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub app_detection_infos: Vec<AppDetectionInfo>,
		#[serde(
			rename = "atsssCapab",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub atsss_capab: Option<AtsssCapability>,
		/// Indicates the DN-AAA authorization profile index
		#[serde(
			rename = "authProfIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_prof_index: Option<String>,
		#[serde(
			rename = "creditManageStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub credit_manage_status: Option<CreditManagementStatus>,
		#[serde(rename = "interGrpIds", default, skip_serializing_if = "Vec::is_empty")]
		pub inter_grp_ids: Vec<GroupId>,
		/// Indicates the invalid parameters for the reported type(s) of the
		/// failed policy decision and/or condition data.
		#[serde(
			rename = "invalidPolicyDecs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_policy_decs: Vec<InvalidParam>,
		/// Indicates the IPv4 address domain
		#[serde(rename = "ipDomain", default, skip_serializing_if = "Option::is_none")]
		pub ip_domain: Option<String>,
		#[serde(
			rename = "ipv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "ipv6AddressPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_address_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "maPduInd", default, skip_serializing_if = "Option::is_none")]
		pub ma_pdu_ind: Option<MaPduIndication>,
		#[serde(
			rename = "mulAddrInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mul_addr_infos: Vec<IpMulticastAddressInfo>,
		/// Contains the number of supported packet filter for signalled QoS
		/// rules.
		#[serde(
			rename = "numOfPackFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_of_pack_filter: Option<i64>,
		#[serde(
			rename = "nwdafDatas",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_datas: Option<Vec<NwdafData>>,
		/// Contains the identifier of the PCC rule which is used for traffic
		/// detection of event.
		#[serde(rename = "pccRuleId", default, skip_serializing_if = "Option::is_none")]
		pub pcc_rule_id: Option<String>,
		#[serde(rename = "pcfUeInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_ue_info: Option<PcfUeCallbackInfo>,
		/// Contains the type(s) of failed policy decision and/or condition
		/// data.
		#[serde(
			rename = "policyDecFailureReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub policy_dec_failure_reports: Vec<PolicyDecisionFailureCode>,
		/// QoS Notification Control information.
		#[serde(rename = "qncReports", default, skip_serializing_if = "Vec::is_empty")]
		pub qnc_reports: Vec<QosNotificationControlInfo>,
		#[serde(
			rename = "qosFlowUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_usage: Option<QosFlowUsage>,
		#[serde(
			rename = "qosMonReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_mon_reports: Vec<QosMonitoringReport>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		/// If it is included and set to true, the reflective QoS is supported
		/// by the UE. If it is included and set to false, the reflective QoS is
		/// revoked by the UE.
		#[serde(
			rename = "refQosIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ref_qos_indication: Option<bool>,
		#[serde(
			rename = "relAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rel_access_info: Option<AdditionalAccessInfo>,
		#[serde(
			rename = "relIpv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rel_ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "relIpv6AddressPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rel_ipv6_address_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "relUeMac", default, skip_serializing_if = "Option::is_none")]
		pub rel_ue_mac: Option<MacAddr48>,
		/// The policy control reqeust trigges which are met.
		#[serde(
			rename = "repPolicyCtrlReqTriggers",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rep_policy_ctrl_req_triggers: Vec<PolicyControlRequestTrigger>,
		/// Reports the changes of presence reporting area. The praId attribute
		/// within the PresenceInfo data type is the key of the map.
		#[serde(
			rename = "repPraInfos",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub rep_pra_infos: ::std::collections::HashMap<String, PresenceInfo>,
		/// Used to report the PCC rule failure.
		#[serde(rename = "ruleReports", default, skip_serializing_if = "Vec::is_empty")]
		pub rule_reports: Vec<RuleReport>,
		#[serde(
			rename = "satBackhaulCategory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sat_backhaul_category: Option<SatelliteBackhaulCategory>,
		#[serde(rename = "servNfId", default, skip_serializing_if = "Option::is_none")]
		pub serv_nf_id: Option<ServingNfIdentity>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		/// Used to report the session rule failure.
		#[serde(
			rename = "sessRuleReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sess_rule_reports: Vec<SessionRuleReport>,
		#[serde(
			rename = "subsDefQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_def_qos: Option<SubscribedDefaultQos>,
		#[serde(
			rename = "subsSessAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_sess_ambr: Option<Ambr>,
		/// If it is included and set to true, the 3GPP PS Data Off is activated
		/// by the UE.
		#[serde(
			rename = "3gppPsDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<bool>,
		#[serde(rename = "traceReq", default, skip_serializing_if = "Option::is_none")]
		pub trace_req: Option<TraceData>,
		#[serde(
			rename = "trafficDescriptors",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub traffic_descriptors: Vec<DddTrafficDescriptor>,
		#[serde(
			rename = "tsnBridgeInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_info: Option<TsnBridgeInfo>,
		#[serde(
			rename = "tsnBridgeManCont",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_bridge_man_cont: Option<BridgeManagementContainer>,
		#[serde(
			rename = "tsnPortManContDstt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsn_port_man_cont_dstt: Option<PortManagementContainer>,
		#[serde(
			rename = "tsnPortManContNwtts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tsn_port_man_cont_nwtts: Vec<PortManagementContainer>,
		#[serde(
			rename = "typesOfNotif",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub types_of_notif: Vec<DlDataDeliveryStatus>,
		#[serde(
			rename = "ueInitResReq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_init_res_req: Option<UeInitiatedResourceRequest>,
		#[serde(rename = "ueMac", default, skip_serializing_if = "Option::is_none")]
		pub ue_mac: Option<MacAddr48>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info: Option<UserLocation>,
		#[serde(
			rename = "userLocationInfoTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info_time: Option<DateTime>,
		#[serde(rename = "vplmnQos", default, skip_serializing_if = "Option::is_none")]
		pub vplmn_qos: Option<VplmnQos>,
		/// If it is included and set to true, indicates that the QoS
		/// constraints in the VPLMN are not applicable.
		#[serde(
			rename = "vplmnQosNotApp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vplmn_qos_not_app: Option<bool>,
	}

	impl From<&SmPolicyUpdateContextData> for SmPolicyUpdateContextData {
		fn from(value: &SmPolicyUpdateContextData) -> Self {
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

	/// Represents a combination of S-NSSAI and DNN(s).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a combination of S-NSSAI and DNN(s).",
	///  "type": "object",
	///  "properties": {
	///    "dnns": {
	///      "type": "array",
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
	pub struct SnssaiDnnCombination {
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub dnns: Vec<Dnn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&SnssaiDnnCombination> for SnssaiDnnCombination {
		fn from(value: &SnssaiDnnCombination) -> Self {
			value.clone()
		}
	}

	/// Describes explicitly the route to an Application location.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes explicitly the route to an Application
	/// location.",
	///  "type": "object",
	///  "required": [
	///    "presenceInfoList"
	///  ],
	///  "properties": {
	///    "presenceInfoList": {
	///      "description": "Defines the presence information provisioned by the
	/// AF. The praId attribute within the PresenceInfo data type is the key of
	/// the map.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SpatialValidity {
		/// Defines the presence information provisioned by the AF. The praId
		/// attribute within the PresenceInfo data type is the key of the map.
		#[serde(rename = "presenceInfoList")]
		pub presence_info_list: ::std::collections::HashMap<String, PresenceInfo>,
	}

	impl From<&SpatialValidity> for SpatialValidity {
		fn from(value: &SpatialValidity) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the SpatialValidity data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// SpatialValidity data type, but with the OpenAPI nullable property set to
	/// true.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "presenceInfoList"
	///  ],
	///  "properties": {
	///    "presenceInfoList": {
	///      "description": "Defines the presence information provisioned by the
	/// AF. The praId attribute within the PresenceInfo data type is the key of
	/// the map.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SpatialValidityRm(pub Option<SpatialValidityRmInner>);

	impl ::std::ops::Deref for SpatialValidityRm {
		type Target = Option<SpatialValidityRmInner>;
		fn deref(&self) -> &Option<SpatialValidityRmInner> {
			&self.0
		}
	}

	impl From<SpatialValidityRm> for Option<SpatialValidityRmInner> {
		fn from(value: SpatialValidityRm) -> Self {
			value.0
		}
	}

	impl From<&SpatialValidityRm> for SpatialValidityRm {
		fn from(value: &SpatialValidityRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<SpatialValidityRmInner>> for SpatialValidityRm {
		fn from(value: Option<SpatialValidityRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the SpatialValidity data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// SpatialValidity data type, but with the OpenAPI nullable property set to
	/// true.",
	///  "type": "object",
	///  "required": [
	///    "presenceInfoList"
	///  ],
	///  "properties": {
	///    "presenceInfoList": {
	///      "description": "Defines the presence information provisioned by the
	/// AF. The praId attribute within the PresenceInfo data type is the key of
	/// the map.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SpatialValidityRmInner {
		/// Defines the presence information provisioned by the AF. The praId
		/// attribute within the PresenceInfo data type is the key of the map.
		#[serde(rename = "presenceInfoList")]
		pub presence_info_list: ::std::collections::HashMap<String, PresenceInfo>,
	}

	impl From<&SpatialValidityRmInner> for SpatialValidityRmInner {
		fn from(value: &SpatialValidityRmInner) -> Self {
			value.clone()
		}
	}

	/// Contains an identity of a sponsor.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an identity of a sponsor.",
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
	pub struct SponId(pub String);

	impl ::std::ops::Deref for SponId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SponId> for String {
		fn from(value: SponId) -> Self {
			value.0
		}
	}

	impl From<&SponId> for SponId {
		fn from(value: &SponId) -> Self {
			value.clone()
		}
	}

	impl From<String> for SponId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SponId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SponId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates whether sponsored data connectivity is enabled or disabled/not
	/// enabled.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether sponsored data connectivity is
	/// enabled or disabled/not enabled.",
	///  "type": "string",
	///  "enum": [
	///    "SPONSOR_DISABLED",
	///    "SPONSOR_ENABLED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SponsoringStatus {
		#[default]
		#[serde(rename = "SPONSOR_DISABLED")]
		SponsorDisabled,
		#[serde(rename = "SPONSOR_ENABLED")]
		SponsorEnabled,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SponsoringStatus> for SponsoringStatus {
		fn from(value: &SponsoringStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for SponsoringStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::SponsorDisabled => "SPONSOR_DISABLED".to_string(),
				Self::SponsorEnabled => "SPONSOR_ENABLED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SponsoringStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SPONSOR_DISABLED" => Ok(Self::SponsorDisabled),
				"SPONSOR_ENABLED" => Ok(Self::SponsorEnabled),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SponsoringStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SponsoringStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SponsoringStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Indicates the time interval(s) during which the AF request is to be
	/// applied.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the time interval(s) during which the AF
	/// request is to be applied.",
	///  "type": "object",
	///  "properties": {
	///    "startTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "stopTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TemporalValidity {
		#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
		pub start_time: Option<DateTime>,
		#[serde(rename = "stopTime", default, skip_serializing_if = "Option::is_none")]
		pub stop_time: Option<DateTime>,
	}

	impl From<&TemporalValidity> for TemporalValidity {
		fn from(value: &TemporalValidity) -> Self {
			value.clone()
		}
	}

	/// Indicates the cause behind requesting the deletion of the Individual
	/// Application Session Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the cause behind requesting the deletion of
	/// the Individual Application Session Context resource.",
	///  "type": "string",
	///  "enum": [
	///    "ALL_SDF_DEACTIVATION",
	///    "PDU_SESSION_TERMINATION",
	///    "PS_TO_CS_HO",
	///    "INSUFFICIENT_SERVER_RESOURCES",
	///    "INSUFFICIENT_QOS_FLOW_RESOURCES",
	///    "SPONSORED_DATA_CONNECTIVITY_DISALLOWED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
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
		#[serde(rename = "ALL_SDF_DEACTIVATION")]
		AllSdfDeactivation,
		#[serde(rename = "PDU_SESSION_TERMINATION")]
		PduSessionTermination,
		#[serde(rename = "PS_TO_CS_HO")]
		PsToCsHo,
		#[serde(rename = "INSUFFICIENT_SERVER_RESOURCES")]
		InsufficientServerResources,
		#[serde(rename = "INSUFFICIENT_QOS_FLOW_RESOURCES")]
		InsufficientQosFlowResources,
		#[serde(rename = "SPONSORED_DATA_CONNECTIVITY_DISALLOWED")]
		SponsoredDataConnectivityDisallowed,
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
				Self::AllSdfDeactivation => "ALL_SDF_DEACTIVATION".to_string(),
				Self::PduSessionTermination => "PDU_SESSION_TERMINATION".to_string(),
				Self::PsToCsHo => "PS_TO_CS_HO".to_string(),
				Self::InsufficientServerResources => "INSUFFICIENT_SERVER_RESOURCES".to_string(),
				Self::InsufficientQosFlowResources => "INSUFFICIENT_QOS_FLOW_RESOURCES".to_string(),
				Self::SponsoredDataConnectivityDisallowed => {
					"SPONSORED_DATA_CONNECTIVITY_DISALLOWED".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TerminationCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ALL_SDF_DEACTIVATION" => Ok(Self::AllSdfDeactivation),
				"PDU_SESSION_TERMINATION" => Ok(Self::PduSessionTermination),
				"PS_TO_CS_HO" => Ok(Self::PsToCsHo),
				"INSUFFICIENT_SERVER_RESOURCES" => Ok(Self::InsufficientServerResources),
				"INSUFFICIENT_QOS_FLOW_RESOURCES" => Ok(Self::InsufficientQosFlowResources),
				"SPONSORED_DATA_CONNECTIVITY_DISALLOWED" => {
					Ok(Self::SponsoredDataConnectivityDisallowed)
				}
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

	/// Indicates the cause for requesting the deletion of the Individual
	/// Application Session Context resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the cause for requesting the deletion of the
	/// Individual Application Session Context resource.",
	///  "type": "object",
	///  "required": [
	///    "resUri",
	///    "termCause"
	///  ],
	///  "properties": {
	///    "resUri": {
	///      "$ref": "#/components/schemas/Uri"
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
	pub struct TerminationInfo {
		#[serde(rename = "resUri")]
		pub res_uri: Uri,
		#[serde(rename = "termCause")]
		pub term_cause: TerminationCause,
	}

	impl From<&TerminationInfo> for TerminationInfo {
		fn from(value: &TerminationInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a request to terminate a policy Association that the PCF
	/// provides in a notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a request to terminate a policy Association
	/// that the PCF provides in a notification.\n",
	///  "type": "object",
	///  "required": [
	///    "cause",
	///    "resourceUri"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/PolicyAssociationReleaseCause"
	///    },
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TerminationNotification {
		pub cause: PolicyAssociationReleaseCause,
		#[serde(rename = "resourceUri")]
		pub resource_uri: Uri,
	}

	impl From<&TerminationNotification> for TerminationNotification {
		fn from(value: &TerminationNotification) -> Self {
			value.clone()
		}
	}

	/// Represents a Termination Notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a Termination Notification.",
	///  "type": "object",
	///  "required": [
	///    "cause",
	///    "resourceUri"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/SmPolicyAssociationReleaseCause"
	///    },
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TerminationNotification1 {
		pub cause: SmPolicyAssociationReleaseCause,
		#[serde(rename = "resourceUri")]
		pub resource_uri: Uri,
	}

	impl From<&TerminationNotification1> for TerminationNotification1 {
		fn from(value: &TerminationNotification1) -> Self {
			value.clone()
		}
	}

	/// Represents a request to terminate a policy association that the PCF
	/// provides in a notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a request to terminate a policy association
	/// that the PCF provides in a notification.\n",
	///  "type": "object",
	///  "required": [
	///    "cause",
	///    "resourceUri"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/PolicyAssociationReleaseCause1"
	///    },
	///    "resourceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TerminationNotification2 {
		pub cause: PolicyAssociationReleaseCause1,
		#[serde(rename = "resourceUri")]
		pub resource_uri: Uri,
	}

	impl From<&TerminationNotification2> for TerminationNotification2 {
		fn from(value: &TerminationNotification2) -> Self {
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

	/// 2-octet string, where each octet is encoded in hexadecimal
	/// representation. The first octet contains the IPv4 Type-of-Service or the
	/// IPv6 Traffic-Class field and the second octet contains the ToS/Traffic
	/// Class mask field.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "2-octet string, where each octet is encoded in
	/// hexadecimal representation. The first octet contains the IPv4
	/// Type-of-Service or the IPv6 Traffic-Class field and the second octet
	/// contains the ToS/Traffic Class mask field.\n",
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
	pub struct TosTrafficClass(pub String);

	impl ::std::ops::Deref for TosTrafficClass {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TosTrafficClass> for String {
		fn from(value: TosTrafficClass) -> Self {
			value.0
		}
	}

	impl From<&TosTrafficClass> for TosTrafficClass {
		fn from(value: &TosTrafficClass) -> Self {
			value.clone()
		}
	}

	impl From<String> for TosTrafficClass {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TosTrafficClass {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for TosTrafficClass {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// This data type is defined in the same way as the TosTrafficClass data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// TosTrafficClass data type, but with the OpenAPI nullable property set to
	/// true.",
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
	pub struct TosTrafficClassRm(pub Option<String>);

	impl ::std::ops::Deref for TosTrafficClassRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<TosTrafficClassRm> for Option<String> {
		fn from(value: TosTrafficClassRm) -> Self {
			value.0
		}
	}

	impl From<&TosTrafficClassRm> for TosTrafficClassRm {
		fn from(value: &TosTrafficClassRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for TosTrafficClassRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Contains parameters determining how flows associated with a PCC Rule are
	/// treated (e.g. blocked, redirected, etc).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains parameters determining how flows associated
	/// with a PCC Rule are treated (e.g. blocked, redirected, etc).",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "tcId"
	///  ],
	///  "properties": {
	///    "addRedirectInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RedirectInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "flowStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "maxAllowedUpLat": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    },
	///    "mulAccCtrl": {
	///      "$ref": "#/components/schemas/MulticastAccessControl"
	///    },
	///    "muteNotif": {
	///      "description": "Indicates whether applicat'on's start or stop
	/// notification is to be muted.",
	///      "type": "boolean"
	///    },
	///    "redirectInfo": {
	///      "$ref": "#/components/schemas/RedirectInformation"
	///    },
	///    "routeToLocs": {
	///      "description": "A list of location which the traffic shall be
	/// routed to for the AF request",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RouteToLocation"
	///      },
	///      "minItems": 1
	///    },
	///    "simConnInd": {
	///      "description": "Indicates whether simultaneous connectivity should
	/// be temporarily maintained for the source and target PSA.",
	///      "type": "boolean"
	///    },
	///    "simConnTerm": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "steerFun": {
	///      "$ref": "#/components/schemas/SteeringFunctionality"
	///    },
	///    "steerModeDl": {
	///      "$ref": "#/components/schemas/SteeringMode"
	///    },
	///    "steerModeUl": {
	///      "$ref": "#/components/schemas/SteeringMode"
	///    },
	///    "tcId": {
	///      "description": "Univocally identifies the traffic control policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "traffCorreInd": {
	///      "type": "boolean"
	///    },
	///    "trafficSteeringPolIdDl": {
	///      "description": "Reference to a pre-configured traffic steering
	/// policy for downlink traffic at the SMF.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "trafficSteeringPolIdUl": {
	///      "description": "Reference to a pre-configured traffic steering
	/// policy for uplink traffic at the SMF.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "upPathChgEvent": {
	///      "$ref": "#/components/schemas/UpPathChgEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TrafficControlData(pub Option<TrafficControlDataInner>);

	impl ::std::ops::Deref for TrafficControlData {
		type Target = Option<TrafficControlDataInner>;
		fn deref(&self) -> &Option<TrafficControlDataInner> {
			&self.0
		}
	}

	impl From<TrafficControlData> for Option<TrafficControlDataInner> {
		fn from(value: TrafficControlData) -> Self {
			value.0
		}
	}

	impl From<&TrafficControlData> for TrafficControlData {
		fn from(value: &TrafficControlData) -> Self {
			value.clone()
		}
	}

	impl From<Option<TrafficControlDataInner>> for TrafficControlData {
		fn from(value: Option<TrafficControlDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains parameters determining how flows associated with a PCC Rule are
	/// treated (e.g. blocked, redirected, etc).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains parameters determining how flows associated
	/// with a PCC Rule are treated (e.g. blocked, redirected, etc).",
	///  "type": "object",
	///  "required": [
	///    "tcId"
	///  ],
	///  "properties": {
	///    "addRedirectInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RedirectInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "flowStatus": {
	///      "$ref": "#/components/schemas/FlowStatus"
	///    },
	///    "maxAllowedUpLat": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    },
	///    "mulAccCtrl": {
	///      "$ref": "#/components/schemas/MulticastAccessControl"
	///    },
	///    "muteNotif": {
	///      "description": "Indicates whether applicat'on's start or stop
	/// notification is to be muted.",
	///      "type": "boolean"
	///    },
	///    "redirectInfo": {
	///      "$ref": "#/components/schemas/RedirectInformation"
	///    },
	///    "routeToLocs": {
	///      "description": "A list of location which the traffic shall be
	/// routed to for the AF request",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/RouteToLocation"
	///      },
	///      "minItems": 1
	///    },
	///    "simConnInd": {
	///      "description": "Indicates whether simultaneous connectivity should
	/// be temporarily maintained for the source and target PSA.",
	///      "type": "boolean"
	///    },
	///    "simConnTerm": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "steerFun": {
	///      "$ref": "#/components/schemas/SteeringFunctionality"
	///    },
	///    "steerModeDl": {
	///      "$ref": "#/components/schemas/SteeringMode"
	///    },
	///    "steerModeUl": {
	///      "$ref": "#/components/schemas/SteeringMode"
	///    },
	///    "tcId": {
	///      "description": "Univocally identifies the traffic control policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "traffCorreInd": {
	///      "type": "boolean"
	///    },
	///    "trafficSteeringPolIdDl": {
	///      "description": "Reference to a pre-configured traffic steering
	/// policy for downlink traffic at the SMF.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "trafficSteeringPolIdUl": {
	///      "description": "Reference to a pre-configured traffic steering
	/// policy for uplink traffic at the SMF.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "upPathChgEvent": {
	///      "$ref": "#/components/schemas/UpPathChgEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TrafficControlDataInner {
		#[serde(
			rename = "addRedirectInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub add_redirect_info: Vec<RedirectInformation>,
		/// Contains EAS IP replacement information.
		#[serde(
			rename = "easIpReplaceInfos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eas_ip_replace_infos: Option<Vec<EasIpReplacementInfo>>,
		#[serde(
			rename = "flowStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub flow_status: Option<FlowStatus>,
		#[serde(
			rename = "maxAllowedUpLat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_allowed_up_lat: Option<UintegerRm>,
		#[serde(
			rename = "mulAccCtrl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mul_acc_ctrl: Option<MulticastAccessControl>,
		/// Indicates whether applicat'on's start or stop notification is to be
		/// muted.
		#[serde(rename = "muteNotif", default, skip_serializing_if = "Option::is_none")]
		pub mute_notif: Option<bool>,
		#[serde(
			rename = "redirectInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redirect_info: Option<RedirectInformation>,
		/// A list of location which the traffic shall be routed to for the AF
		/// request
		#[serde(
			rename = "routeToLocs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub route_to_locs: Option<Vec<RouteToLocation>>,
		/// Indicates whether simultaneous connectivity should be temporarily
		/// maintained for the source and target PSA.
		#[serde(
			rename = "simConnInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_ind: Option<bool>,
		#[serde(
			rename = "simConnTerm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sim_conn_term: Option<DurationSec>,
		#[serde(rename = "steerFun", default, skip_serializing_if = "Option::is_none")]
		pub steer_fun: Option<SteeringFunctionality>,
		#[serde(
			rename = "steerModeDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steer_mode_dl: Option<SteeringMode>,
		#[serde(
			rename = "steerModeUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steer_mode_ul: Option<SteeringMode>,
		/// Univocally identifies the traffic control policy data within a PDU
		/// session.
		#[serde(rename = "tcId")]
		pub tc_id: String,
		#[serde(
			rename = "traffCorreInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traff_corre_ind: Option<bool>,
		/// Reference to a pre-configured traffic steering policy for downlink
		/// traffic at the SMF.
		#[serde(
			rename = "trafficSteeringPolIdDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_steering_pol_id_dl: Option<String>,
		/// Reference to a pre-configured traffic steering policy for uplink
		/// traffic at the SMF.
		#[serde(
			rename = "trafficSteeringPolIdUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_steering_pol_id_ul: Option<String>,
		#[serde(
			rename = "upPathChgEvent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_path_chg_event: Option<UpPathChgEvent>,
	}

	impl From<&TrafficControlDataInner> for TrafficControlDataInner {
		fn from(value: &TrafficControlDataInner) -> Self {
			value.clone()
		}
	}

	/// Identify a traffic descriptor as defined in Figure 5.2.2 of 3GPP TS
	/// 24.526, octets v+5 to w.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identify a traffic descriptor as defined in Figure
	/// 5.2.2 of 3GPP TS 24.526, octets v+5 to w.",
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
	pub struct TrafficDescriptor(pub String);

	impl ::std::ops::Deref for TrafficDescriptor {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TrafficDescriptor> for String {
		fn from(value: TrafficDescriptor) -> Self {
			value.0
		}
	}

	impl From<&TrafficDescriptor> for TrafficDescriptor {
		fn from(value: &TrafficDescriptor) -> Self {
			value.clone()
		}
	}

	impl From<String> for TrafficDescriptor {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TrafficDescriptor {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for TrafficDescriptor {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Describes a transfer policy.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes a transfer policy.",
	///  "type": "object",
	///  "required": [
	///    "ratingGroup",
	///    "recTimeInt",
	///    "transPolicyId"
	///  ],
	///  "properties": {
	///    "maxBitRateDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxBitRateUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "ratingGroup": {
	///      "description": "Indicates a rating group for the recommended time
	/// window.",
	///      "type": "integer"
	///    },
	///    "recTimeInt": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "transPolicyId": {
	///      "description": "Contains an identity of a transfer policy.",
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransferPolicy {
		#[serde(
			rename = "maxBitRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_bit_rate_dl: Option<BitRate>,
		#[serde(
			rename = "maxBitRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_bit_rate_ul: Option<BitRate>,
		/// Indicates a rating group for the recommended time window.
		#[serde(rename = "ratingGroup")]
		pub rating_group: i64,
		#[serde(rename = "recTimeInt")]
		pub rec_time_int: TimeWindow,
		/// Contains an identity of a transfer policy.
		#[serde(rename = "transPolicyId")]
		pub trans_policy_id: i64,
	}

	impl From<&TransferPolicy> for TransferPolicy {
		fn from(value: &TransferPolicy) -> Self {
			value.clone()
		}
	}

	/// Represents the priority level of TSC Flows.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the priority level of TSC Flows.",
	///  "type": "integer",
	///  "maximum": 8.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TscPriorityLevel(pub i64);

	impl ::std::ops::Deref for TscPriorityLevel {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<TscPriorityLevel> for i64 {
		fn from(value: TscPriorityLevel) -> Self {
			value.0
		}
	}

	impl From<&TscPriorityLevel> for TscPriorityLevel {
		fn from(value: &TscPriorityLevel) -> Self {
			value.clone()
		}
	}

	impl From<i64> for TscPriorityLevel {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TscPriorityLevel {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for TscPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TscPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TscPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for TscPriorityLevel {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// This data type is defined in the same way as the TscPriorityLevel data
	/// type, but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// TscPriorityLevel data type, but with the OpenAPI nullable property set
	/// to true.",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 8.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TscPriorityLevelRm(pub Option<i64>);

	impl ::std::ops::Deref for TscPriorityLevelRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<TscPriorityLevelRm> for Option<i64> {
		fn from(value: TscPriorityLevelRm) -> Self {
			value.0
		}
	}

	impl From<&TscPriorityLevelRm> for TscPriorityLevelRm {
		fn from(value: &TscPriorityLevelRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for TscPriorityLevelRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// Indicates TSC Traffic pattern.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates TSC Traffic pattern.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "burstArrivalTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "periodicity": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInNumMsg": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInTime": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TscaiInputContainer(pub Option<TscaiInputContainerInner>);

	impl ::std::ops::Deref for TscaiInputContainer {
		type Target = Option<TscaiInputContainerInner>;
		fn deref(&self) -> &Option<TscaiInputContainerInner> {
			&self.0
		}
	}

	impl From<TscaiInputContainer> for Option<TscaiInputContainerInner> {
		fn from(value: TscaiInputContainer) -> Self {
			value.0
		}
	}

	impl From<&TscaiInputContainer> for TscaiInputContainer {
		fn from(value: &TscaiInputContainer) -> Self {
			value.clone()
		}
	}

	impl From<Option<TscaiInputContainerInner>> for TscaiInputContainer {
		fn from(value: Option<TscaiInputContainerInner>) -> Self {
			Self(value)
		}
	}

	/// Indicates TSC Traffic pattern.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates TSC Traffic pattern.",
	///  "type": "object",
	///  "properties": {
	///    "burstArrivalTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "periodicity": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInNumMsg": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "surTimeInTime": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TscaiInputContainerInner {
		#[serde(
			rename = "burstArrivalTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub burst_arrival_time: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub periodicity: Option<Uinteger>,
		#[serde(
			rename = "surTimeInNumMsg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sur_time_in_num_msg: Option<Uinteger>,
		#[serde(
			rename = "surTimeInTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sur_time_in_time: Option<Uinteger>,
	}

	impl From<&TscaiInputContainerInner> for TscaiInputContainerInner {
		fn from(value: &TscaiInputContainerInner) -> Self {
			value.clone()
		}
	}

	/// Contains parameters that describe and identify the TSC user plane node.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains parameters that describe and identify the TSC
	/// user plane node.",
	///  "type": "object",
	///  "properties": {
	///    "bridgeId": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "dsttAddr": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "dsttPortNum": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "dsttResidTime": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TsnBridgeInfo {
		#[serde(rename = "bridgeId", default, skip_serializing_if = "Option::is_none")]
		pub bridge_id: Option<Uint64>,
		#[serde(rename = "dsttAddr", default, skip_serializing_if = "Option::is_none")]
		pub dstt_addr: Option<MacAddr48>,
		#[serde(
			rename = "dsttPortNum",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dstt_port_num: Option<Uinteger>,
		#[serde(
			rename = "dsttResidTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dstt_resid_time: Option<Uinteger>,
	}

	impl From<&TsnBridgeInfo> for TsnBridgeInfo {
		fn from(value: &TsnBridgeInfo) -> Self {
			value.clone()
		}
	}

	/// TsnPortNumber
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
	pub struct TsnPortNumber(pub Uinteger);

	impl ::std::ops::Deref for TsnPortNumber {
		type Target = Uinteger;
		fn deref(&self) -> &Uinteger {
			&self.0
		}
	}

	impl From<TsnPortNumber> for Uinteger {
		fn from(value: TsnPortNumber) -> Self {
			value.0
		}
	}

	impl From<&TsnPortNumber> for TsnPortNumber {
		fn from(value: &TsnPortNumber) -> Self {
			value.clone()
		}
	}

	impl From<Uinteger> for TsnPortNumber {
		fn from(value: Uinteger) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TsnPortNumber {
		type Err = <Uinteger as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for TsnPortNumber {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TsnPortNumber {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TsnPortNumber {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for TsnPortNumber {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates TSC Traffic QoS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates TSC Traffic QoS.",
	///  "type": "object",
	///  "properties": {
	///    "maxTscBurstSize": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVol"
	///    },
	///    "tscPackDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "tscPrioLevel": {
	///      "$ref": "#/components/schemas/TscPriorityLevel"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TsnQosContainer {
		#[serde(
			rename = "maxTscBurstSize",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_tsc_burst_size: Option<ExtMaxDataBurstVol>,
		#[serde(
			rename = "tscPackDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsc_pack_delay: Option<PacketDelBudget>,
		#[serde(
			rename = "tscPrioLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsc_prio_level: Option<TscPriorityLevel>,
	}

	impl From<&TsnQosContainer> for TsnQosContainer {
		fn from(value: &TsnQosContainer) -> Self {
			value.clone()
		}
	}

	/// Indicates removable TSC Traffic QoS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates removable TSC Traffic QoS.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "maxTscBurstSize": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVolRm"
	///    },
	///    "tscPackDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudgetRm"
	///    },
	///    "tscPrioLevel": {
	///      "$ref": "#/components/schemas/TscPriorityLevelRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TsnQosContainerRm(pub Option<TsnQosContainerRmInner>);

	impl ::std::ops::Deref for TsnQosContainerRm {
		type Target = Option<TsnQosContainerRmInner>;
		fn deref(&self) -> &Option<TsnQosContainerRmInner> {
			&self.0
		}
	}

	impl From<TsnQosContainerRm> for Option<TsnQosContainerRmInner> {
		fn from(value: TsnQosContainerRm) -> Self {
			value.0
		}
	}

	impl From<&TsnQosContainerRm> for TsnQosContainerRm {
		fn from(value: &TsnQosContainerRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<TsnQosContainerRmInner>> for TsnQosContainerRm {
		fn from(value: Option<TsnQosContainerRmInner>) -> Self {
			Self(value)
		}
	}

	/// Indicates removable TSC Traffic QoS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates removable TSC Traffic QoS.",
	///  "type": "object",
	///  "properties": {
	///    "maxTscBurstSize": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVolRm"
	///    },
	///    "tscPackDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudgetRm"
	///    },
	///    "tscPrioLevel": {
	///      "$ref": "#/components/schemas/TscPriorityLevelRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TsnQosContainerRmInner {
		#[serde(
			rename = "maxTscBurstSize",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_tsc_burst_size: Option<ExtMaxDataBurstVolRm>,
		#[serde(
			rename = "tscPackDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsc_pack_delay: Option<PacketDelBudgetRm>,
		#[serde(
			rename = "tscPrioLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tsc_prio_level: Option<TscPriorityLevelRm>,
	}

	impl From<&TsnQosContainerRmInner> for TsnQosContainerRmInner {
		fn from(value: &TsnQosContainerRmInner) -> Self {
			value.clone()
		}
	}

	/// Contains the current applicable values corresponding to the policy
	/// control request triggers.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the current applicable values corresponding to
	/// the policy control request triggers.",
	///  "type": "object",
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "netLocAccSupp": {
	///      "$ref": "#/components/schemas/NetLocAccessSupport"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "satBackhaulCategory": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "servNfId": {
	///      "$ref": "#/components/schemas/ServingNfIdentity"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeCampingRep {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(
			rename = "netLocAccSupp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub net_loc_acc_supp: Option<NetLocAccessSupport>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "satBackhaulCategory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sat_backhaul_category: Option<SatelliteBackhaulCategory>,
		#[serde(rename = "servNfId", default, skip_serializing_if = "Option::is_none")]
		pub serv_nf_id: Option<ServingNfIdentity>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info: Option<UserLocation>,
	}

	impl From<&UeCampingRep> for UeCampingRep {
		fn from(value: &UeCampingRep) -> Self {
			value.clone()
		}
	}

	/// Represents 5GS-Level UE identities.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents 5GS-Level UE identities.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "gpsi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "pei"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "supi"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
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
	#[serde(untagged)]
	pub enum UeIdentityInfo {
		#[default]
		Variant0 {
			gpsi: Gpsi,
		},
		Variant1 {
			pei: Pei,
		},
		Variant2 {
			supi: Supi,
		},
	}

	impl From<&UeIdentityInfo> for UeIdentityInfo {
		fn from(value: &UeIdentityInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates that a UE requests specific QoS handling for the selected SDF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates that a UE requests specific QoS handling for
	/// the selected SDF.",
	///  "type": "object",
	///  "required": [
	///    "packFiltInfo",
	///    "ruleOp"
	///  ],
	///  "properties": {
	///    "packFiltInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PacketFilterInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "pccRuleId": {
	///      "type": "string"
	///    },
	///    "precedence": {
	///      "type": "integer"
	///    },
	///    "reqQos": {
	///      "$ref": "#/components/schemas/RequestedQos"
	///    },
	///    "ruleOp": {
	///      "$ref": "#/components/schemas/RuleOperation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeInitiatedResourceRequest {
		#[serde(rename = "packFiltInfo")]
		pub pack_filt_info: Vec<PacketFilterInfo>,
		#[serde(rename = "pccRuleId", default, skip_serializing_if = "Option::is_none")]
		pub pcc_rule_id: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub precedence: Option<i64>,
		#[serde(rename = "reqQos", default, skip_serializing_if = "Option::is_none")]
		pub req_qos: Option<RequestedQos>,
		#[serde(rename = "ruleOp")]
		pub rule_op: RuleOperation,
	}

	impl From<&UeInitiatedResourceRequest> for UeInitiatedResourceRequest {
		fn from(value: &UeInitiatedResourceRequest) -> Self {
			value.clone()
		}
	}

	/// UePolicy
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
	pub struct UePolicy(pub Bytes);

	impl ::std::ops::Deref for UePolicy {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UePolicy> for Bytes {
		fn from(value: UePolicy) -> Self {
			value.0
		}
	}

	impl From<&UePolicy> for UePolicy {
		fn from(value: &UePolicy) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UePolicy {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UePolicy {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UePolicy {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UePolicy {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UePolicy {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UePolicy {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// UePolicyDeliveryResult
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
	pub struct UePolicyDeliveryResult(pub Bytes);

	impl ::std::ops::Deref for UePolicyDeliveryResult {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UePolicyDeliveryResult> for Bytes {
		fn from(value: UePolicyDeliveryResult) -> Self {
			value.0
		}
	}

	impl From<&UePolicyDeliveryResult> for UePolicyDeliveryResult {
		fn from(value: &UePolicyDeliveryResult) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UePolicyDeliveryResult {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UePolicyDeliveryResult {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UePolicyDeliveryResult {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UePolicyDeliveryResult {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UePolicyDeliveryResult {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UePolicyDeliveryResult {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// UePolicyRequest
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
	pub struct UePolicyRequest(pub Bytes);

	impl ::std::ops::Deref for UePolicyRequest {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UePolicyRequest> for Bytes {
		fn from(value: UePolicyRequest) -> Self {
			value.0
		}
	}

	impl From<&UePolicyRequest> for UePolicyRequest {
		fn from(value: &UePolicyRequest) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UePolicyRequest {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UePolicyRequest {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UePolicyRequest {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UePolicyRequest {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UePolicyRequest {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UePolicyRequest {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents information on the failure of a UE policy transfer to the UE
	/// because the UE is not reachable.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information on the failure of a UE policy
	/// transfer to the UE because the UE is not reachable.\n",
	///  "type": "object",
	///  "required": [
	///    "cause",
	///    "ptis"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/N1N2MessageTransferCause"
	///    },
	///    "ptis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uinteger"
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
	pub struct UePolicyTransferFailureNotification {
		pub cause: N1n2MessageTransferCause,
		pub ptis: Vec<Uinteger>,
	}

	impl From<&UePolicyTransferFailureNotification> for UePolicyTransferFailureNotification {
		fn from(value: &UePolicyTransferFailureNotification) -> Self {
			value.clone()
		}
	}

	/// Contains the current applicable values corresponding to the policy
	/// control request triggers.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the current applicable values corresponding to
	/// the policy control request triggers.\n",
	///  "type": "object",
	///  "properties": {
	///    "connectState": {
	///      "$ref": "#/components/schemas/CmState"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "praStatuses": {
	///      "description": "Contains the UE presence statuses for tracking
	/// areas. The praId attribute within the PresenceInfo data type is the key
	/// of the map.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "userLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeRequestedValueRep {
		#[serde(
			rename = "connectState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub connect_state: Option<CmState>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnIdNid>,
		/// Contains the UE presence statuses for tracking areas. The praId
		/// attribute within the PresenceInfo data type is the key of the map.
		#[serde(
			rename = "praStatuses",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_statuses: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "userLoc", default, skip_serializing_if = "Option::is_none")]
		pub user_loc: Option<UserLocation>,
	}

	impl From<&UeRequestedValueRep> for UeRequestedValueRep {
		fn from(value: &UeRequestedValueRep) -> Self {
			value.clone()
		}
	}

	/// Contains a UE-Slice-MBR and the related information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a UE-Slice-MBR and the related information.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "servingSnssai",
	///    "sliceMbr"
	///  ],
	///  "properties": {
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "servingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "sliceMbr": {
	///      "description": "Contains the MBR for uplink and the MBR for
	/// downlink.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SliceMbr"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeSliceMbr(pub Option<UeSliceMbrInner>);

	impl ::std::ops::Deref for UeSliceMbr {
		type Target = Option<UeSliceMbrInner>;
		fn deref(&self) -> &Option<UeSliceMbrInner> {
			&self.0
		}
	}

	impl From<UeSliceMbr> for Option<UeSliceMbrInner> {
		fn from(value: UeSliceMbr) -> Self {
			value.0
		}
	}

	impl From<&UeSliceMbr> for UeSliceMbr {
		fn from(value: &UeSliceMbr) -> Self {
			value.clone()
		}
	}

	impl From<Option<UeSliceMbrInner>> for UeSliceMbr {
		fn from(value: Option<UeSliceMbrInner>) -> Self {
			Self(value)
		}
	}

	/// Contains a UE-Slice-MBR and the related information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a UE-Slice-MBR and the related information.",
	///  "type": "object",
	///  "required": [
	///    "servingSnssai",
	///    "sliceMbr"
	///  ],
	///  "properties": {
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "servingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "sliceMbr": {
	///      "description": "Contains the MBR for uplink and the MBR for
	/// downlink.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SliceMbr"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeSliceMbrInner {
		#[serde(
			rename = "mappedHomeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapped_home_snssai: Option<Snssai>,
		#[serde(rename = "servingSnssai")]
		pub serving_snssai: Snssai,
		/// Contains the MBR for uplink and the MBR for downlink.
		#[serde(rename = "sliceMbr")]
		pub slice_mbr: ::std::collections::HashMap<String, SliceMbr>,
	}

	impl From<&UeSliceMbrInner> for UeSliceMbrInner {
		fn from(value: &UeSliceMbrInner) -> Self {
			value.clone()
		}
	}

	/// Contains the UP path change event subscription from the AF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UP path change event subscription from the
	/// AF.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "dnaiChgType",
	///    "notifCorreId",
	///    "notificationUri"
	///  ],
	///  "properties": {
	///    "afAckInd": {
	///      "type": "boolean"
	///    },
	///    "dnaiChgType": {
	///      "$ref": "#/components/schemas/DnaiChangeType"
	///    },
	///    "notifCorreId": {
	///      "description": "It is used to set the value of Notification
	/// Correlation ID in the notification sent by the SMF.",
	///      "type": "string"
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpPathChgEvent(pub Option<UpPathChgEventInner>);

	impl ::std::ops::Deref for UpPathChgEvent {
		type Target = Option<UpPathChgEventInner>;
		fn deref(&self) -> &Option<UpPathChgEventInner> {
			&self.0
		}
	}

	impl From<UpPathChgEvent> for Option<UpPathChgEventInner> {
		fn from(value: UpPathChgEvent) -> Self {
			value.0
		}
	}

	impl From<&UpPathChgEvent> for UpPathChgEvent {
		fn from(value: &UpPathChgEvent) -> Self {
			value.clone()
		}
	}

	impl From<Option<UpPathChgEventInner>> for UpPathChgEvent {
		fn from(value: Option<UpPathChgEventInner>) -> Self {
			Self(value)
		}
	}

	/// Contains the UP path change event subscription from the AF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UP path change event subscription from the
	/// AF.",
	///  "type": "object",
	///  "required": [
	///    "dnaiChgType",
	///    "notifCorreId",
	///    "notificationUri"
	///  ],
	///  "properties": {
	///    "afAckInd": {
	///      "type": "boolean"
	///    },
	///    "dnaiChgType": {
	///      "$ref": "#/components/schemas/DnaiChangeType"
	///    },
	///    "notifCorreId": {
	///      "description": "It is used to set the value of Notification
	/// Correlation ID in the notification sent by the SMF.",
	///      "type": "string"
	///    },
	///    "notificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpPathChgEventInner {
		#[serde(rename = "afAckInd", default, skip_serializing_if = "Option::is_none")]
		pub af_ack_ind: Option<bool>,
		#[serde(rename = "dnaiChgType")]
		pub dnai_chg_type: DnaiChangeType,
		/// It is used to set the value of Notification Correlation ID in the
		/// notification sent by the SMF.
		#[serde(rename = "notifCorreId")]
		pub notif_corre_id: String,
		#[serde(rename = "notificationUri")]
		pub notification_uri: Uri,
	}

	impl From<&UpPathChgEventInner> for UpPathChgEventInner {
		fn from(value: &UpPathChgEventInner) -> Self {
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

	/// Contains usage monitoring related control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains usage monitoring related control
	/// information.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "umId"
	///  ],
	///  "properties": {
	///    "exUsagePccRuleIds": {
	///      "description": "Contains the PCC rule identifier(s) which
	/// corresponding service data flow(s) shall be excluded from PDU Session
	/// usage monitoring. It is only included in the UsageMonitoringData
	/// instance for session level usage monitoring.\n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "inactivityTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "monitoringTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "nextTimeThreshold": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "nextVolThreshold": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "nextVolThresholdDownlink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "nextVolThresholdUplink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "timeThreshold": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "umId": {
	///      "description": "Univocally identifies the usage monitoring policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "volumeThreshold": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "volumeThresholdDownlink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "volumeThresholdUplink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UsageMonitoringData(pub Option<UsageMonitoringDataInner>);

	impl ::std::ops::Deref for UsageMonitoringData {
		type Target = Option<UsageMonitoringDataInner>;
		fn deref(&self) -> &Option<UsageMonitoringDataInner> {
			&self.0
		}
	}

	impl From<UsageMonitoringData> for Option<UsageMonitoringDataInner> {
		fn from(value: UsageMonitoringData) -> Self {
			value.0
		}
	}

	impl From<&UsageMonitoringData> for UsageMonitoringData {
		fn from(value: &UsageMonitoringData) -> Self {
			value.clone()
		}
	}

	impl From<Option<UsageMonitoringDataInner>> for UsageMonitoringData {
		fn from(value: Option<UsageMonitoringDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains usage monitoring related control information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains usage monitoring related control
	/// information.",
	///  "type": "object",
	///  "required": [
	///    "umId"
	///  ],
	///  "properties": {
	///    "exUsagePccRuleIds": {
	///      "description": "Contains the PCC rule identifier(s) which
	/// corresponding service data flow(s) shall be excluded from PDU Session
	/// usage monitoring. It is only included in the UsageMonitoringData
	/// instance for session level usage monitoring.\n",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "inactivityTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "monitoringTime": {
	///      "$ref": "#/components/schemas/DateTimeRm"
	///    },
	///    "nextTimeThreshold": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "nextVolThreshold": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "nextVolThresholdDownlink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "nextVolThresholdUplink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "timeThreshold": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "umId": {
	///      "description": "Univocally identifies the usage monitoring policy
	/// data within a PDU session.",
	///      "type": "string"
	///    },
	///    "volumeThreshold": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "volumeThresholdDownlink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "volumeThresholdUplink": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UsageMonitoringDataInner {
		/// Contains the PCC rule identifier(s) which corresponding service data
		/// flow(s) shall be excluded from PDU Session usage monitoring. It is
		/// only included in the UsageMonitoringData instance for session level
		/// usage monitoring.
		#[serde(
			rename = "exUsagePccRuleIds",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ex_usage_pcc_rule_ids: Option<Vec<String>>,
		#[serde(
			rename = "inactivityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inactivity_time: Option<DurationSecRm>,
		#[serde(
			rename = "monitoringTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub monitoring_time: Option<DateTimeRm>,
		#[serde(
			rename = "nextTimeThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_time_threshold: Option<DurationSecRm>,
		#[serde(
			rename = "nextVolThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_threshold: Option<VolumeRm>,
		#[serde(
			rename = "nextVolThresholdDownlink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_threshold_downlink: Option<VolumeRm>,
		#[serde(
			rename = "nextVolThresholdUplink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_vol_threshold_uplink: Option<VolumeRm>,
		#[serde(
			rename = "timeThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_threshold: Option<DurationSecRm>,
		/// Univocally identifies the usage monitoring policy data within a PDU
		/// session.
		#[serde(rename = "umId")]
		pub um_id: String,
		#[serde(
			rename = "volumeThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_threshold: Option<VolumeRm>,
		#[serde(
			rename = "volumeThresholdDownlink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_threshold_downlink: Option<VolumeRm>,
		#[serde(
			rename = "volumeThresholdUplink",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_threshold_uplink: Option<VolumeRm>,
	}

	impl From<&UsageMonitoringDataInner> for UsageMonitoringDataInner {
		fn from(value: &UsageMonitoringDataInner) -> Self {
			value.clone()
		}
	}

	/// Represents a usage threshold.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a usage threshold.",
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "duration": {
	///      "$ref": "#/components/schemas/schemas-DurationSec"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Volume"
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
	pub struct UsageThreshold {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Volume>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub duration: Option<SchemasDurationSec>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Volume>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Volume>,
	}

	impl From<&UsageThreshold> for UsageThreshold {
		fn from(value: &UsageThreshold) -> Self {
			value.clone()
		}
	}

	/// Represents the same as the UsageThreshold data type but with the
	/// nullable:true property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the same as the UsageThreshold data type but
	/// with the nullable:true property.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "duration": {
	///      "$ref": "#/components/schemas/schemas-DurationSecRm"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UsageThresholdRm(pub Option<UsageThresholdRmInner>);

	impl ::std::ops::Deref for UsageThresholdRm {
		type Target = Option<UsageThresholdRmInner>;
		fn deref(&self) -> &Option<UsageThresholdRmInner> {
			&self.0
		}
	}

	impl From<UsageThresholdRm> for Option<UsageThresholdRmInner> {
		fn from(value: UsageThresholdRm) -> Self {
			value.0
		}
	}

	impl From<&UsageThresholdRm> for UsageThresholdRm {
		fn from(value: &UsageThresholdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<UsageThresholdRmInner>> for UsageThresholdRm {
		fn from(value: Option<UsageThresholdRmInner>) -> Self {
			Self(value)
		}
	}

	/// Represents the same as the UsageThreshold data type but with the
	/// nullable:true property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the same as the UsageThreshold data type but
	/// with the nullable:true property.",
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "duration": {
	///      "$ref": "#/components/schemas/schemas-DurationSecRm"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/VolumeRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UsageThresholdRmInner {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<VolumeRm>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub duration: Option<SchemasDurationSecRm>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<VolumeRm>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<VolumeRm>,
	}

	impl From<&UsageThresholdRmInner> for UsageThresholdRmInner {
		fn from(value: &UsageThresholdRmInner) -> Self {
			value.clone()
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

	/// Unsigned integer identifying a volume in units of bytes with
	/// "nullable=true" property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a volume in units of bytes
	/// with \"nullable=true\" property.",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "format": "int64",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VolumeRm(pub Option<i64>);

	impl ::std::ops::Deref for VolumeRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<VolumeRm> for Option<i64> {
		fn from(value: VolumeRm) -> Self {
			value.0
		}
	}

	impl From<&VolumeRm> for VolumeRm {
		fn from(value: &VolumeRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for VolumeRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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

	/// _5gSmCause
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
	pub struct _5gSmCause(pub Uinteger);

	impl ::std::ops::Deref for _5gSmCause {
		type Target = Uinteger;
		fn deref(&self) -> &Uinteger {
			&self.0
		}
	}

	impl From<_5gSmCause> for Uinteger {
		fn from(value: _5gSmCause) -> Self {
			value.0
		}
	}

	impl From<&_5gSmCause> for _5gSmCause {
		fn from(value: &_5gSmCause) -> Self {
			value.clone()
		}
	}

	impl From<Uinteger> for _5gSmCause {
		fn from(value: Uinteger) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _5gSmCause {
		type Err = <Uinteger as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for _5gSmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5gSmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5gSmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for _5gSmCause {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}
}
