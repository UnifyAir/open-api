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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
}
