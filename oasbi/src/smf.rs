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
}
