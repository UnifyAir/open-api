#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};


#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	pub use crate::common::*;

	/// AccessAndMobilitySubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "3gppChargingCharacteristics": {
	///      "$ref": "#/components/schemas/3GppChargingCharacteristics"
	///    },
	///    "activeTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "adjacentPlmnRestrictions": {
	///      "description": "A map (list of key-value pairs where PlmnId serves
	/// as key) of PlmnRestriction",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PlmnRestriction"
	///      }
	///    },
	///    "aerialUeSubInfo": {
	///      "$ref": "#/components/schemas/AerialUeSubscriptionInfo"
	///    },
	///    "cMsisdn": {
	///      "$ref": "#/components/schemas/CMsisdn"
	///    },
	///    "cagData": {
	///      "$ref": "#/components/schemas/CagData"
	///    },
	///    "coreNetworkTypeRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CoreNetworkType"
	///      }
	///    },
	///    "ecRestrictionDataNb": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecRestrictionDataWb": {
	///      "$ref": "#/components/schemas/EcRestrictionDataWb"
	///    },
	///    "edrxParametersList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EdrxParameters"
	///      },
	///      "minItems": 1
	///    },
	///    "expectedUeBehaviourList": {
	///      "$ref": "#/components/schemas/ExpectedUeBehaviourData"
	///    },
	///    "forbiddenAreas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Area"
	///      }
	///    },
	///    "gpsis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      }
	///    },
	///    "hssGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "iabOperationAllowed": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "internalGroupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "mcsPriority": {
	///      "$ref": "#/components/schemas/McsPriorityIndicator"
	///    },
	///    "mdtConfiguration": {
	///      "$ref": "#/components/schemas/MdtConfiguration"
	///    },
	///    "mdtUserConsent": {
	///      "$ref": "#/components/schemas/MdtUserConsent"
	///    },
	///    "micoAllowed": {
	///      "$ref": "#/components/schemas/MicoAllowed"
	///    },
	///    "mpsPriority": {
	///      "$ref": "#/components/schemas/MpsPriorityIndicator"
	///    },
	///    "nbIoTUePriority": {
	///      "$ref": "#/components/schemas/NbIoTUePriority"
	///    },
	///    "nssai": {
	///      "$ref": "#/components/schemas/Nssai"
	///    },
	///    "nssaiInclusionAllowed": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "odbPacketServices": {
	///      "$ref": "#/components/schemas/OdbPacketServices"
	///    },
	///    "pcfSelectionAssistanceInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PcfSelectionAssistanceInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "primaryRatRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "ptwParametersList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PtwParameters"
	///      },
	///      "minItems": 1
	///    },
	///    "ratRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "remoteProvInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "rfspIndex": {
	///      "$ref": "#/components/schemas/RfspIndexRm"
	///    },
	///    "rgWirelineCharacteristics": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "roamingRestrictions": {
	///      "$ref": "#/components/schemas/RoamingRestrictions"
	///    },
	///    "routingIndicator": {
	///      "type": "string",
	///      "pattern": "^[0-9]{1,4}$"
	///    },
	///    "secondaryRatRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "serviceAreaRestriction": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "serviceGapTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sharedAmDataIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataId"
	///      },
	///      "minItems": 1
	///    },
	///    "sharedVnGroupDataIds": {
	///      "description": "A map(list of key-value pairs) where GroupId serves
	/// as key of SharedDataId",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SharedDataId"
	///      }
	///    },
	///    "sorInfo": {
	///      "$ref": "#/components/schemas/SorInfo"
	///    },
	///    "sorInfoExpectInd": {
	///      "type": "boolean"
	///    },
	///    "sorUpdateIndicatorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SorUpdateIndicator"
	///      },
	///      "minItems": 1
	///    },
	///    "sorafRetrieval": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "stnSr": {
	///      "$ref": "#/components/schemas/StnSr"
	///    },
	///    "subsRegTimer": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "subscribedDnnList": {
	///      "type": "array",
	///      "items": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/Dnn"
	///          },
	///          {
	///            "$ref": "#/components/schemas/WildcardDnn"
	///          }
	///        ]
	///      }
	///    },
	///    "subscribedUeAmbr": {
	///      "$ref": "#/components/schemas/AmbrRm"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "ueUsageType": {
	///      "$ref": "#/components/schemas/UeUsageType"
	///    },
	///    "upuInfo": {
	///      "$ref": "#/components/schemas/UpuInfo"
	///    },
	///    "wirelineForbiddenAreas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WirelineArea"
	///      }
	///    },
	///    "wirelineServiceAreaRestriction": {
	///      "$ref": "#/components/schemas/WirelineServiceAreaRestriction"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessAndMobilitySubscriptionData {
		#[serde(
			rename = "activeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub active_time: Option<DurationSecRm>,
		/// A map (list of key-value pairs where PlmnId serves as key) of
		/// PlmnRestriction
		#[serde(
			rename = "adjacentPlmnRestrictions",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub adjacent_plmn_restrictions: ::std::collections::HashMap<String, PlmnRestriction>,
		#[serde(
			rename = "aerialUeSubInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aerial_ue_sub_info: Option<AerialUeSubscriptionInfo>,
		#[serde(rename = "cMsisdn", default, skip_serializing_if = "Option::is_none")]
		pub c_msisdn: Option<CMsisdn>,
		#[serde(rename = "cagData", default, skip_serializing_if = "Option::is_none")]
		pub cag_data: Option<CagData>,
		#[serde(
			rename = "coreNetworkTypeRestrictions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub core_network_type_restrictions: Vec<CoreNetworkType>,
		#[serde(rename = "ecRestrictionDataNb", default)]
		pub ec_restriction_data_nb: bool,
		#[serde(
			rename = "ecRestrictionDataWb",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ec_restriction_data_wb: Option<EcRestrictionDataWb>,
		#[serde(
			rename = "edrxParametersList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub edrx_parameters_list: Vec<EdrxParameters>,
		#[serde(
			rename = "expectedUeBehaviourList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expected_ue_behaviour_list: Option<ExpectedUeBehaviourData>,
		#[serde(
			rename = "forbiddenAreas",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forbidden_areas: Vec<Area>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub gpsis: Vec<Gpsi>,
		#[serde(
			rename = "hssGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hss_group_id: Option<NfGroupId>,
		#[serde(rename = "iabOperationAllowed", default)]
		pub iab_operation_allowed: bool,
		#[serde(
			rename = "internalGroupIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_ids: Vec<GroupId>,
		#[serde(
			rename = "mcsPriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mcs_priority: Option<McsPriorityIndicator>,
		#[serde(
			rename = "mdtConfiguration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mdt_configuration: Option<MdtConfiguration>,
		#[serde(
			rename = "mdtUserConsent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mdt_user_consent: Option<MdtUserConsent>,
		#[serde(
			rename = "micoAllowed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mico_allowed: Option<MicoAllowed>,
		#[serde(
			rename = "mpsPriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mps_priority: Option<MpsPriorityIndicator>,
		#[serde(
			rename = "nbIoTUePriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nb_io_t_ue_priority: Option<NbIoTUePriority>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nssai: Option<Nssai>,
		#[serde(rename = "nssaiInclusionAllowed", default)]
		pub nssai_inclusion_allowed: bool,
		#[serde(
			rename = "odbPacketServices",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub odb_packet_services: Option<OdbPacketServices>,
		#[serde(
			rename = "pcfSelectionAssistanceInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pcf_selection_assistance_infos: Vec<PcfSelectionAssistanceInfo>,
		#[serde(
			rename = "primaryRatRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub primary_rat_restrictions: Option<Vec<RatType>>,
		#[serde(
			rename = "ptwParametersList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ptw_parameters_list: Vec<PtwParameters>,
		#[serde(
			rename = "ratRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rat_restrictions: Option<Vec<RatType>>,
		#[serde(rename = "remoteProvInd", default)]
		pub remote_prov_ind: bool,
		#[serde(rename = "rfspIndex", default, skip_serializing_if = "Option::is_none")]
		pub rfsp_index: Option<RfspIndexRm>,
		#[serde(
			rename = "rgWirelineCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rg_wireline_characteristics: Option<Bytes>,
		#[serde(
			rename = "roamingRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_restrictions: Option<RoamingRestrictions>,
		#[serde(
			rename = "routingIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub routing_indicator: Option<AccessAndMobilitySubscriptionDataRoutingIndicator>,
		#[serde(
			rename = "secondaryRatRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_rat_restrictions: Option<Vec<RatType>>,
		#[serde(
			rename = "serviceAreaRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_area_restriction: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "serviceGapTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_gap_time: Option<DurationSec>,
		#[serde(
			rename = "sharedAmDataIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub shared_am_data_ids: Vec<SharedDataId>,
		/// A map(list of key-value pairs) where GroupId serves as key of
		/// SharedDataId
		#[serde(
			rename = "sharedVnGroupDataIds",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub shared_vn_group_data_ids: ::std::collections::HashMap<String, SharedDataId>,
		#[serde(rename = "sorInfo", default, skip_serializing_if = "Option::is_none")]
		pub sor_info: Option<SorInfo>,
		#[serde(
			rename = "sorInfoExpectInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_info_expect_ind: Option<bool>,
		#[serde(
			rename = "sorUpdateIndicatorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sor_update_indicator_list: Vec<SorUpdateIndicator>,
		#[serde(rename = "sorafRetrieval", default)]
		pub soraf_retrieval: bool,
		#[serde(rename = "stnSr", default, skip_serializing_if = "Option::is_none")]
		pub stn_sr: Option<StnSr>,
		#[serde(
			rename = "subsRegTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_reg_timer: Option<DurationSecRm>,
		#[serde(
			rename = "subscribedDnnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub subscribed_dnn_list: Vec<AccessAndMobilitySubscriptionDataSubscribedDnnListItem>,
		#[serde(
			rename = "subscribedUeAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribed_ue_ambr: Option<AmbrRm>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "3gppChargingCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_charging_characteristics: Option<_3gppChargingCharacteristics>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(
			rename = "ueUsageType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_usage_type: Option<UeUsageType>,
		#[serde(rename = "upuInfo", default, skip_serializing_if = "Option::is_none")]
		pub upu_info: Option<UpuInfo>,
		#[serde(
			rename = "wirelineForbiddenAreas",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub wireline_forbidden_areas: Vec<WirelineArea>,
		#[serde(
			rename = "wirelineServiceAreaRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wireline_service_area_restriction: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&AccessAndMobilitySubscriptionData> for AccessAndMobilitySubscriptionData {
		fn from(value: &AccessAndMobilitySubscriptionData) -> Self {
			value.clone()
		}
	}

	/// AccessAndMobilitySubscriptionDataRoutingIndicator
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
		NewUnchecked
	)]
	pub struct AccessAndMobilitySubscriptionDataRoutingIndicator(String);

	impl ::std::ops::Deref for AccessAndMobilitySubscriptionDataRoutingIndicator {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AccessAndMobilitySubscriptionDataRoutingIndicator> for String {
		fn from(value: AccessAndMobilitySubscriptionDataRoutingIndicator) -> Self {
			value.0
		}
	}

	impl From<&AccessAndMobilitySubscriptionDataRoutingIndicator>
		for AccessAndMobilitySubscriptionDataRoutingIndicator
	{
		fn from(value: &AccessAndMobilitySubscriptionDataRoutingIndicator) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AccessAndMobilitySubscriptionDataRoutingIndicator {
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

	impl ::std::convert::TryFrom<&str> for AccessAndMobilitySubscriptionDataRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AccessAndMobilitySubscriptionDataRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AccessAndMobilitySubscriptionDataRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AccessAndMobilitySubscriptionDataRoutingIndicator {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// AccessAndMobilitySubscriptionDataSubscribedDnnListItem
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
	pub struct AccessAndMobilitySubscriptionDataSubscribedDnnListItem {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&AccessAndMobilitySubscriptionDataSubscribedDnnListItem>
		for AccessAndMobilitySubscriptionDataSubscribedDnnListItem
	{
		fn from(value: &AccessAndMobilitySubscriptionDataSubscribedDnnListItem) -> Self {
			value.clone()
		}
	}

	/// AccessNetworkId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "HRPD",
	///    "WIMAX",
	///    "WLAN",
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
	pub enum AccessNetworkId {
		#[default]
		#[serde(rename = "HRPD")]
		Hrpd,
		#[serde(rename = "WIMAX")]
		Wimax,
		#[serde(rename = "WLAN")]
		Wlan,
		#[serde(rename = "ETHERNET")]
		Ethernet,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AccessNetworkId> for AccessNetworkId {
		fn from(value: &AccessNetworkId) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessNetworkId {
		fn to_string(&self) -> String {
			match *self {
				Self::Hrpd => "HRPD".to_string(),
				Self::Wimax => "WIMAX".to_string(),
				Self::Wlan => "WLAN".to_string(),
				Self::Ethernet => "ETHERNET".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AccessNetworkId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"HRPD" => Ok(Self::Hrpd),
				"WIMAX" => Ok(Self::Wimax),
				"WLAN" => Ok(Self::Wlan),
				"ETHERNET" => Ok(Self::Ethernet),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessNetworkId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessNetworkId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessNetworkId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

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

	/// AcknowledgeInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "provisioningTime"
	///  ],
	///  "properties": {
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sorMacIue": {
	///      "$ref": "#/components/schemas/SorMac"
	///    },
	///    "sorTransparentContainer": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "ueNotReachable": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "upuMacIue": {
	///      "$ref": "#/components/schemas/UpuMac"
	///    },
	///    "upuTransparentContainer": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AcknowledgeInfo {
		#[serde(rename = "provisioningTime")]
		pub provisioning_time: DateTime,
		#[serde(rename = "sorMacIue", default, skip_serializing_if = "Option::is_none")]
		pub sor_mac_iue: Option<SorMac>,
		#[serde(
			rename = "sorTransparentContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_transparent_container: Option<Bytes>,
		#[serde(rename = "ueNotReachable", default)]
		pub ue_not_reachable: bool,
		#[serde(rename = "upuMacIue", default, skip_serializing_if = "Option::is_none")]
		pub upu_mac_iue: Option<UpuMac>,
		#[serde(
			rename = "upuTransparentContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upu_transparent_container: Option<Bytes>,
	}

	impl From<&AcknowledgeInfo> for AcknowledgeInfo {
		fn from(value: &AcknowledgeInfo) -> Self {
			value.clone()
		}
	}

	/// AdditionalSnssaiData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "requiredAuthnAuthz": {
	///      "type": "boolean"
	///    },
	///    "subscribedNsSrgList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsSrg"
	///      },
	///      "minItems": 1
	///    },
	///    "subscribedUeSliceMbr": {
	///      "$ref": "#/components/schemas/SliceMbrRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AdditionalSnssaiData {
		#[serde(
			rename = "requiredAuthnAuthz",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub required_authn_authz: Option<bool>,
		#[serde(
			rename = "subscribedNsSrgList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub subscribed_ns_srg_list: Vec<NsSrg>,
		#[serde(
			rename = "subscribedUeSliceMbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribed_ue_slice_mbr: Option<SliceMbrRm>,
	}

	impl From<&AdditionalSnssaiData> for AdditionalSnssaiData {
		fn from(value: &AdditionalSnssaiData) -> Self {
			value.clone()
		}
	}

	/// Indicates the Aerial service for the UE is allowed or not allowed,
	/// possible values are - AERIAL_UE_ALLOWED: Aerial service for the UE is
	/// allowed. - AERIAL_UE_NOT_ALLOWED: Aerial service for the UE is not
	/// allowed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the Aerial service for the UE is allowed or
	/// not allowed, possible values are - AERIAL_UE_ALLOWED: Aerial service for
	/// the UE is allowed. - AERIAL_UE_NOT_ALLOWED: Aerial service for the UE is
	/// not allowed.\n",
	///  "type": "string",
	///  "enum": [
	///    "AERIAL_UE_ALLOWED",
	///    "AERIAL_UE_NOT_ALLOWED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AerialUeIndication {
		#[default]
		#[serde(rename = "AERIAL_UE_ALLOWED")]
		AerialUeAllowed,
		#[serde(rename = "AERIAL_UE_NOT_ALLOWED")]
		AerialUeNotAllowed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AerialUeIndication> for AerialUeIndication {
		fn from(value: &AerialUeIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for AerialUeIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::AerialUeAllowed => "AERIAL_UE_ALLOWED".to_string(),
				Self::AerialUeNotAllowed => "AERIAL_UE_NOT_ALLOWED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AerialUeIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AERIAL_UE_ALLOWED" => Ok(Self::AerialUeAllowed),
				"AERIAL_UE_NOT_ALLOWED" => Ok(Self::AerialUeNotAllowed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AerialUeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AerialUeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AerialUeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the Aerial UE Subscription Information, it at least contains
	/// the Aerial UE Indication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Aerial UE Subscription Information, it at
	/// least contains the Aerial UE Indication.",
	///  "type": "object",
	///  "required": [
	///    "aerialUeInd"
	///  ],
	///  "properties": {
	///    "3gppUavId": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "aerialUeInd": {
	///      "$ref": "#/components/schemas/AerialUeIndication"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AerialUeSubscriptionInfo {
		#[serde(rename = "aerialUeInd")]
		pub aerial_ue_ind: AerialUeIndication,
		#[serde(rename = "3gppUavId", default, skip_serializing_if = "Option::is_none")]
		pub three_gpp_uav_id: Option<Gpsi>,
	}

	impl From<&AerialUeSubscriptionInfo> for AerialUeSubscriptionInfo {
		fn from(value: &AerialUeSubscriptionInfo) -> Self {
			value.clone()
		}
	}

	/// AfExternal
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "afId": {
	///      "$ref": "#/components/schemas/AfId"
	///    },
	///    "allowedGeographicArea": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 1
	///    },
	///    "privacyCheckRelatedAction": {
	///      "$ref": "#/components/schemas/PrivacyCheckRelatedAction"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfExternal {
		#[serde(rename = "afId", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<AfId>,
		#[serde(
			rename = "allowedGeographicArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_geographic_area: Vec<GeographicArea>,
		#[serde(
			rename = "privacyCheckRelatedAction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub privacy_check_related_action: Option<PrivacyCheckRelatedAction>,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&AfExternal> for AfExternal {
		fn from(value: &AfExternal) -> Self {
			value.clone()
		}
	}

	/// AfId
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
	pub struct AfId(pub String);

	impl ::std::ops::Deref for AfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AfId> for String {
		fn from(value: AfId) -> Self {
			value.0
		}
	}

	impl From<&AfId> for AfId {
		fn from(value: &AfId) -> Self {
			value.clone()
		}
	}

	impl From<String> for AfId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AfId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for AfId {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Amf3GppAccessRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "amfInstanceId",
	///    "deregCallbackUri",
	///    "guami",
	///    "ratType"
	///  ],
	///  "properties": {
	///    "adminDeregSubWithdrawn": {
	///      "type": "boolean"
	///    },
	///    "amfEeSubscriptionId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "amfServiceNameDereg": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "amfServiceNamePcscfRest": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "backupAmfInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "deregCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "drFlag": {
	///      "$ref": "#/components/schemas/DualRegistrationFlag"
	///    },
	///    "emergencyRegistrationInd": {
	///      "type": "boolean"
	///    },
	///    "epsInterworkingInfo": {
	///      "$ref": "#/components/schemas/EpsInterworkingInfo"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "imsVoPs": {
	///      "$ref": "#/components/schemas/ImsVoPs"
	///    },
	///    "initialRegistrationInd": {
	///      "type": "boolean"
	///    },
	///    "lastSynchronizationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "noEeSubscriptionInd": {
	///      "type": "boolean"
	///    },
	///    "pcscfRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "purgeFlag": {
	///      "$ref": "#/components/schemas/PurgeFlag"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "reRegistrationRequired": {
	///      "type": "boolean"
	///    },
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sorSnpnSiSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueMINTCapability": {
	///      "type": "boolean"
	///    },
	///    "ueReachableInd": {
	///      "$ref": "#/components/schemas/UeReachableInd"
	///    },
	///    "ueSrvccCapability": {
	///      "type": "boolean"
	///    },
	///    "urrpIndicator": {
	///      "type": "boolean"
	///    },
	///    "vgmlcAddress": {
	///      "$ref": "#/components/schemas/VgmlcAddress"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Amf3GppAccessRegistration {
		#[serde(
			rename = "adminDeregSubWithdrawn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub admin_dereg_sub_withdrawn: Option<bool>,
		#[serde(
			rename = "amfEeSubscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_ee_subscription_id: Option<Uri>,
		#[serde(rename = "amfInstanceId")]
		pub amf_instance_id: NfInstanceId,
		#[serde(
			rename = "amfServiceNameDereg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_service_name_dereg: Option<ServiceName>,
		#[serde(
			rename = "amfServiceNamePcscfRest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_service_name_pcscf_rest: Option<ServiceName>,
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_amf_info: Vec<BackupAmfInfo>,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(rename = "deregCallbackUri")]
		pub dereg_callback_uri: Uri,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		#[serde(rename = "drFlag", default, skip_serializing_if = "Option::is_none")]
		pub dr_flag: Option<DualRegistrationFlag>,
		#[serde(
			rename = "emergencyRegistrationInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub emergency_registration_ind: Option<bool>,
		#[serde(
			rename = "epsInterworkingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_info: Option<EpsInterworkingInfo>,
		pub guami: Guami,
		#[serde(rename = "imsVoPs", default, skip_serializing_if = "Option::is_none")]
		pub ims_vo_ps: Option<ImsVoPs>,
		#[serde(
			rename = "initialRegistrationInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub initial_registration_ind: Option<bool>,
		#[serde(
			rename = "lastSynchronizationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_synchronization_time: Option<DateTime>,
		#[serde(
			rename = "noEeSubscriptionInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_ee_subscription_ind: Option<bool>,
		#[serde(
			rename = "pcscfRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcscf_restoration_callback_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "purgeFlag", default, skip_serializing_if = "Option::is_none")]
		pub purge_flag: Option<PurgeFlag>,
		#[serde(rename = "ratType")]
		pub rat_type: RatType,
		#[serde(
			rename = "reRegistrationRequired",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub re_registration_required: Option<bool>,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "sorSnpnSiSupported", default)]
		pub sor_snpn_si_supported: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
		#[serde(
			rename = "ueMINTCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_mint_capability: Option<bool>,
		#[serde(
			rename = "ueReachableInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_reachable_ind: Option<UeReachableInd>,
		#[serde(
			rename = "ueSrvccCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_srvcc_capability: Option<bool>,
		#[serde(
			rename = "urrpIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub urrp_indicator: Option<bool>,
		#[serde(
			rename = "vgmlcAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vgmlc_address: Option<VgmlcAddress>,
	}

	impl From<&Amf3GppAccessRegistration> for Amf3GppAccessRegistration {
		fn from(value: &Amf3GppAccessRegistration) -> Self {
			value.clone()
		}
	}

	/// Amf3GppAccessRegistrationModification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "guami"
	///  ],
	///  "properties": {
	///    "backupAmfInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      }
	///    },
	///    "epsInterworkingInfo": {
	///      "$ref": "#/components/schemas/EpsInterworkingInfo"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "imsVoPs": {
	///      "$ref": "#/components/schemas/ImsVoPs"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "purgeFlag": {
	///      "$ref": "#/components/schemas/PurgeFlag"
	///    },
	///    "ueMINTCapability": {
	///      "type": "boolean"
	///    },
	///    "ueSrvccCapability": {
	///      "type": [
	///        "boolean",
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
	pub struct Amf3GppAccessRegistrationModification {
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_amf_info: Vec<BackupAmfInfo>,
		#[serde(
			rename = "epsInterworkingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_info: Option<EpsInterworkingInfo>,
		pub guami: Guami,
		#[serde(rename = "imsVoPs", default, skip_serializing_if = "Option::is_none")]
		pub ims_vo_ps: Option<ImsVoPs>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "purgeFlag", default, skip_serializing_if = "Option::is_none")]
		pub purge_flag: Option<PurgeFlag>,
		#[serde(
			rename = "ueMINTCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_mint_capability: Option<bool>,
		#[serde(
			rename = "ueSrvccCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_srvcc_capability: Option<bool>,
	}

	impl From<&Amf3GppAccessRegistrationModification> for Amf3GppAccessRegistrationModification {
		fn from(value: &Amf3GppAccessRegistrationModification) -> Self {
			value.clone()
		}
	}

	/// AmfDeregInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "deregReason"
	///  ],
	///  "properties": {
	///    "deregReason": {
	///      "$ref": "#/components/schemas/DeregistrationReason"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfDeregInfo {
		#[serde(rename = "deregReason")]
		pub dereg_reason: DeregistrationReason,
	}

	impl From<&AmfDeregInfo> for AmfDeregInfo {
		fn from(value: &AmfDeregInfo) -> Self {
			value.clone()
		}
	}

	/// AmfInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "amfInstanceId",
	///    "guami"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfInfo {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(rename = "amfInstanceId")]
		pub amf_instance_id: NfInstanceId,
		pub guami: Guami,
	}

	impl From<&AmfInfo> for AmfInfo {
		fn from(value: &AmfInfo) -> Self {
			value.clone()
		}
	}

	/// AmfNon3GppAccessRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "amfInstanceId",
	///    "deregCallbackUri",
	///    "guami",
	///    "imsVoPs",
	///    "ratType"
	///  ],
	///  "properties": {
	///    "adminDeregSubWithdrawn": {
	///      "type": "boolean"
	///    },
	///    "amfEeSubscriptionId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "amfServiceNameDereg": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "amfServiceNamePcscfRest": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "backupAmfInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "deregCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "imsVoPs": {
	///      "$ref": "#/components/schemas/ImsVoPs"
	///    },
	///    "lastSynchronizationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "noEeSubscriptionInd": {
	///      "type": "boolean"
	///    },
	///    "pcscfRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "purgeFlag": {
	///      "$ref": "#/components/schemas/PurgeFlag"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "reRegistrationRequired": {
	///      "type": "boolean"
	///    },
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sorSnpnSiSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "urrpIndicator": {
	///      "type": "boolean"
	///    },
	///    "vgmlcAddress": {
	///      "$ref": "#/components/schemas/VgmlcAddress"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfNon3GppAccessRegistration {
		#[serde(
			rename = "adminDeregSubWithdrawn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub admin_dereg_sub_withdrawn: Option<bool>,
		#[serde(
			rename = "amfEeSubscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_ee_subscription_id: Option<Uri>,
		#[serde(rename = "amfInstanceId")]
		pub amf_instance_id: NfInstanceId,
		#[serde(
			rename = "amfServiceNameDereg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_service_name_dereg: Option<ServiceName>,
		#[serde(
			rename = "amfServiceNamePcscfRest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_service_name_pcscf_rest: Option<ServiceName>,
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_amf_info: Vec<BackupAmfInfo>,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(rename = "deregCallbackUri")]
		pub dereg_callback_uri: Uri,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		pub guami: Guami,
		#[serde(rename = "imsVoPs")]
		pub ims_vo_ps: ImsVoPs,
		#[serde(
			rename = "lastSynchronizationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_synchronization_time: Option<DateTime>,
		#[serde(
			rename = "noEeSubscriptionInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_ee_subscription_ind: Option<bool>,
		#[serde(
			rename = "pcscfRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcscf_restoration_callback_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "purgeFlag", default, skip_serializing_if = "Option::is_none")]
		pub purge_flag: Option<PurgeFlag>,
		#[serde(rename = "ratType")]
		pub rat_type: RatType,
		#[serde(
			rename = "reRegistrationRequired",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub re_registration_required: Option<bool>,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "sorSnpnSiSupported", default)]
		pub sor_snpn_si_supported: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
		#[serde(
			rename = "urrpIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub urrp_indicator: Option<bool>,
		#[serde(
			rename = "vgmlcAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vgmlc_address: Option<VgmlcAddress>,
	}

	impl From<&AmfNon3GppAccessRegistration> for AmfNon3GppAccessRegistration {
		fn from(value: &AmfNon3GppAccessRegistration) -> Self {
			value.clone()
		}
	}

	/// AmfNon3GppAccessRegistrationModification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "guami"
	///  ],
	///  "properties": {
	///    "backupAmfInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      }
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "imsVoPs": {
	///      "$ref": "#/components/schemas/ImsVoPs"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "purgeFlag": {
	///      "$ref": "#/components/schemas/PurgeFlag"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfNon3GppAccessRegistrationModification {
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_amf_info: Vec<BackupAmfInfo>,
		pub guami: Guami,
		#[serde(rename = "imsVoPs", default, skip_serializing_if = "Option::is_none")]
		pub ims_vo_ps: Option<ImsVoPs>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(rename = "purgeFlag", default, skip_serializing_if = "Option::is_none")]
		pub purge_flag: Option<PurgeFlag>,
	}

	impl From<&AmfNon3GppAccessRegistrationModification> for AmfNon3GppAccessRegistrationModification {
		fn from(value: &AmfNon3GppAccessRegistrationModification) -> Self {
			value.clone()
		}
	}

	/// AppDescriptor
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "appId": {
	///      "type": "string"
	///    },
	///    "osId": {
	///      "$ref": "#/components/schemas/OsId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppDescriptor {
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<String>,
		#[serde(rename = "osId", default, skip_serializing_if = "Option::is_none")]
		pub os_id: Option<OsId>,
	}

	impl From<&AppDescriptor> for AppDescriptor {
		fn from(value: &AppDescriptor) -> Self {
			value.clone()
		}
	}

	/// AppPortId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "destinationPort": {
	///      "$ref": "#/components/schemas/Uint16"
	///    },
	///    "originatorPort": {
	///      "$ref": "#/components/schemas/Uint16"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppPortId {
		#[serde(
			rename = "destinationPort",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub destination_port: Option<Uint16>,
		#[serde(
			rename = "originatorPort",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_port: Option<Uint16>,
	}

	impl From<&AppPortId> for AppPortId {
		fn from(value: &AppPortId) -> Self {
			value.clone()
		}
	}

	/// AssociationType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "IMEI_CHANGE",
	///    "IMEISV_CHANGE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AssociationType {
		#[default]
		#[serde(rename = "IMEI_CHANGE")]
		ImeiChange,
		#[serde(rename = "IMEISV_CHANGE")]
		ImeisvChange,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AssociationType> for AssociationType {
		fn from(value: &AssociationType) -> Self {
			value.clone()
		}
	}

	impl ToString for AssociationType {
		fn to_string(&self) -> String {
			match *self {
				Self::ImeiChange => "IMEI_CHANGE".to_string(),
				Self::ImeisvChange => "IMEISV_CHANGE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AssociationType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IMEI_CHANGE" => Ok(Self::ImeiChange),
				"IMEISV_CHANGE" => Ok(Self::ImeisvChange),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AssociationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AssociationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AssociationType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AuthEvent
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "authType",
	///    "nfInstanceId",
	///    "servingNetworkName",
	///    "success",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "authRemovalInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "authType": {
	///      "$ref": "#/components/schemas/AuthType"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
	///    },
	///    "success": {
	///      "$ref": "#/components/schemas/Success"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "udrRestartInd": {
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
	pub struct AuthEvent {
		#[serde(rename = "authRemovalInd", default)]
		pub auth_removal_ind: bool,
		#[serde(rename = "authType")]
		pub auth_type: AuthType,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
		pub nf_set_id: Option<NfSetId>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "servingNetworkName")]
		pub serving_network_name: ServingNetworkName,
		pub success: Success,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
	}

	impl From<&AuthEvent> for AuthEvent {
		fn from(value: &AuthEvent) -> Self {
			value.clone()
		}
	}

	/// AuthType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "5G_AKA",
	///    "EAP_AKA_PRIME",
	///    "EAP_TLS",
	///    "NONE",
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
		#[serde(rename = "NONE")]
		None,
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
				Self::None => "NONE".to_string(),
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
				"NONE" => Ok(Self::None),
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

	/// Represents authorization update information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents authorization update information.",
	///  "type": "object",
	///  "required": [
	///    "authorizationData"
	///  ],
	///  "properties": {
	///    "authorizationData": {
	///      "$ref": "#/components/schemas/ServiceSpecificAuthorizationData"
	///    },
	///    "invalidCause": {
	///      "$ref": "#/components/schemas/InvalidCause"
	///    },
	///    "invalidityInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthUpdateInfo {
		#[serde(rename = "authorizationData")]
		pub authorization_data: ServiceSpecificAuthorizationData,
		#[serde(
			rename = "invalidCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invalid_cause: Option<InvalidCause>,
		#[serde(
			rename = "invalidityInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invalidity_ind: Option<bool>,
	}

	impl From<&AuthUpdateInfo> for AuthUpdateInfo {
		fn from(value: &AuthUpdateInfo) -> Self {
			value.clone()
		}
	}

	/// Represents an authorization update notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an authorization update notification.",
	///  "type": "object",
	///  "required": [
	///    "authUpdateInfoList",
	///    "serviceType"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "authUpdateInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AuthUpdateInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "serviceType": {
	///      "$ref": "#/components/schemas/ServiceType"
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
	pub struct AuthUpdateNotification {
		#[serde(rename = "afId", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<String>,
		#[serde(rename = "authUpdateInfoList")]
		pub auth_update_info_list: Vec<AuthUpdateInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "serviceType")]
		pub service_type: ServiceType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&AuthUpdateNotification> for AuthUpdateNotification {
		fn from(value: &AuthUpdateNotification) -> Self {
			value.clone()
		}
	}

	/// AuthenticatedInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthenticatedInd(pub bool);

	impl ::std::ops::Deref for AuthenticatedInd {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<AuthenticatedInd> for bool {
		fn from(value: AuthenticatedInd) -> Self {
			value.0
		}
	}

	impl From<&AuthenticatedInd> for AuthenticatedInd {
		fn from(value: &AuthenticatedInd) -> Self {
			value.clone()
		}
	}

	impl From<bool> for AuthenticatedInd {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AuthenticatedInd {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AuthenticatedInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AuthenticatedInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AuthenticatedInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AuthenticatedInd {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// AuthenticationInfoRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "ausfInstanceId",
	///    "servingNetworkName"
	///  ],
	///  "properties": {
	///    "ausfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
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
	///    "resynchronizationInfo": {
	///      "$ref": "#/components/schemas/ResynchronizationInfo"
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
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
	pub struct AuthenticationInfoRequest {
		#[serde(rename = "ausfInstanceId")]
		pub ausf_instance_id: NfInstanceId,
		#[serde(rename = "cellCagInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub cell_cag_info: Vec<CagId>,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		#[serde(rename = "n5gcInd", default)]
		pub n5gc_ind: bool,
		#[serde(rename = "nswoInd", default)]
		pub nswo_ind: bool,
		#[serde(
			rename = "resynchronizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resynchronization_info: Option<ResynchronizationInfo>,
		#[serde(rename = "servingNetworkName")]
		pub serving_network_name: ServingNetworkName,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&AuthenticationInfoRequest> for AuthenticationInfoRequest {
		fn from(value: &AuthenticationInfoRequest) -> Self {
			value.clone()
		}
	}

	/// AuthenticationInfoResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "authType"
	///  ],
	///  "properties": {
	///    "akmaInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "authAaa": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "authType": {
	///      "$ref": "#/components/schemas/AuthType"
	///    },
	///    "authenticationVector": {
	///      "$ref": "#/components/schemas/AuthenticationVector"
	///    },
	///    "pvsInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServerAddressingInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "routingId": {
	///      "type": "string",
	///      "pattern": "^[0-9]{1,4}$"
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
	pub struct AuthenticationInfoResult {
		#[serde(rename = "akmaInd", default)]
		pub akma_ind: bool,
		#[serde(rename = "authAaa", default)]
		pub auth_aaa: bool,
		#[serde(rename = "authType")]
		pub auth_type: AuthType,
		#[serde(
			rename = "authenticationVector",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub authentication_vector: Option<AuthenticationVector>,
		#[serde(rename = "pvsInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pvs_info: Vec<ServerAddressingInfo>,
		#[serde(rename = "routingId", default, skip_serializing_if = "Option::is_none")]
		pub routing_id: Option<AuthenticationInfoResultRoutingId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&AuthenticationInfoResult> for AuthenticationInfoResult {
		fn from(value: &AuthenticationInfoResult) -> Self {
			value.clone()
		}
	}

	/// AuthenticationInfoResultRoutingId
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
		NewUnchecked
	)]
	pub struct AuthenticationInfoResultRoutingId(String);

	impl ::std::ops::Deref for AuthenticationInfoResultRoutingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AuthenticationInfoResultRoutingId> for String {
		fn from(value: AuthenticationInfoResultRoutingId) -> Self {
			value.0
		}
	}

	impl From<&AuthenticationInfoResultRoutingId> for AuthenticationInfoResultRoutingId {
		fn from(value: &AuthenticationInfoResultRoutingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AuthenticationInfoResultRoutingId {
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

	impl ::std::convert::TryFrom<&str> for AuthenticationInfoResultRoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AuthenticationInfoResultRoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AuthenticationInfoResultRoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AuthenticationInfoResultRoutingId {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// AuthenticationVector
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/AvEapAkaPrime"
	///    },
	///    {
	///      "$ref": "#/components/schemas/Av5GHeAka"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AuthenticationVector {
		#[default]
		AvEapAkaPrime(AvEapAkaPrime),
		Av5GHeAka(Av5GHeAka),
	}

	impl From<&AuthenticationVector> for AuthenticationVector {
		fn from(value: &AuthenticationVector) -> Self {
			value.clone()
		}
	}

	impl From<AvEapAkaPrime> for AuthenticationVector {
		fn from(value: AvEapAkaPrime) -> Self {
			Self::AvEapAkaPrime(value)
		}
	}

	impl From<Av5GHeAka> for AuthenticationVector {
		fn from(value: Av5GHeAka) -> Self {
			Self::Av5GHeAka(value)
		}
	}

	/// Represents NIDD authorization data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents NIDD authorization data.",
	///  "type": "object",
	///  "required": [
	///    "authorizationData"
	///  ],
	///  "properties": {
	///    "authorizationData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UserIdentifier"
	///      },
	///      "minItems": 1,
	///      "uniqueItems": true
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
	pub struct AuthorizationData {
		#[serde(rename = "authorizationData")]
		pub authorization_data: Vec<UserIdentifier>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&AuthorizationData> for AuthorizationData {
		fn from(value: &AuthorizationData) -> Self {
			value.clone()
		}
	}

	/// Represents NIDD authorization information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents NIDD authorization information.",
	///  "type": "object",
	///  "required": [
	///    "authUpdateCallbackUri",
	///    "dnn",
	///    "mtcProviderInformation",
	///    "snssai"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "authUpdateCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "nefId": {
	///      "$ref": "#/components/schemas/NefId"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
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
	pub struct AuthorizationInfo {
		#[serde(rename = "afId", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<String>,
		#[serde(rename = "authUpdateCallbackUri")]
		pub auth_update_callback_uri: Uri,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		pub dnn: Dnn,
		#[serde(rename = "mtcProviderInformation")]
		pub mtc_provider_information: MtcProviderInformation,
		#[serde(rename = "nefId", default, skip_serializing_if = "Option::is_none")]
		pub nef_id: Option<NefId>,
		pub snssai: Snssai,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&AuthorizationInfo> for AuthorizationInfo {
		fn from(value: &AuthorizationInfo) -> Self {
			value.clone()
		}
	}

	/// UE Id of the Authorization Data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE Id of the Authorization Data.",
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
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
	pub struct AuthorizationUeId {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		pub supi: Supi,
	}

	impl From<&AuthorizationUeId> for AuthorizationUeId {
		fn from(value: &AuthorizationUeId) -> Self {
			value.clone()
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
		NewUnchecked
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
		NewUnchecked
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

	/// Av5GHeAka
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "avType",
	///    "kausf",
	///    "rand",
	///    "xresStar"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "avType": {
	///      "$ref": "#/components/schemas/AvType"
	///    },
	///    "kausf": {
	///      "$ref": "#/components/schemas/Kausf"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    },
	///    "xresStar": {
	///      "$ref": "#/components/schemas/XresStar"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Av5GHeAka {
		pub autn: Autn,
		#[serde(rename = "avType")]
		pub av_type: AvType,
		pub kausf: Kausf,
		pub rand: Rand,
		#[serde(rename = "xresStar")]
		pub xres_star: XresStar,
	}

	impl From<&Av5GHeAka> for Av5GHeAka {
		fn from(value: &Av5GHeAka) -> Self {
			value.clone()
		}
	}

	/// AvEapAkaPrime
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "avType",
	///    "ckPrime",
	///    "ikPrime",
	///    "rand",
	///    "xres"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "avType": {
	///      "$ref": "#/components/schemas/AvType"
	///    },
	///    "ckPrime": {
	///      "$ref": "#/components/schemas/CkPrime"
	///    },
	///    "ikPrime": {
	///      "$ref": "#/components/schemas/IkPrime"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    },
	///    "xres": {
	///      "$ref": "#/components/schemas/Xres"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AvEapAkaPrime {
		pub autn: Autn,
		#[serde(rename = "avType")]
		pub av_type: AvType,
		#[serde(rename = "ckPrime")]
		pub ck_prime: CkPrime,
		#[serde(rename = "ikPrime")]
		pub ik_prime: IkPrime,
		pub rand: Rand,
		pub xres: Xres,
	}

	impl From<&AvEapAkaPrime> for AvEapAkaPrime {
		fn from(value: &AvEapAkaPrime) -> Self {
			value.clone()
		}
	}

	/// AvEpsAka
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "avType",
	///    "kasme",
	///    "rand",
	///    "xres"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "avType": {
	///      "$ref": "#/components/schemas/HssAvType"
	///    },
	///    "kasme": {
	///      "$ref": "#/components/schemas/Kasme"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    },
	///    "xres": {
	///      "$ref": "#/components/schemas/Xres"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AvEpsAka {
		pub autn: Autn,
		#[serde(rename = "avType")]
		pub av_type: HssAvType,
		pub kasme: Kasme,
		pub rand: Rand,
		pub xres: Xres,
	}

	impl From<&AvEpsAka> for AvEpsAka {
		fn from(value: &AvEpsAka) -> Self {
			value.clone()
		}
	}

	/// AvImsGbaEapAka
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "avType",
	///    "ck",
	///    "ik",
	///    "rand",
	///    "xres"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "avType": {
	///      "$ref": "#/components/schemas/HssAvType"
	///    },
	///    "ck": {
	///      "$ref": "#/components/schemas/ConfidentialityKey"
	///    },
	///    "ik": {
	///      "$ref": "#/components/schemas/IntegrityKey"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    },
	///    "xres": {
	///      "$ref": "#/components/schemas/Xres"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AvImsGbaEapAka {
		pub autn: Autn,
		#[serde(rename = "avType")]
		pub av_type: HssAvType,
		pub ck: ConfidentialityKey,
		pub ik: IntegrityKey,
		pub rand: Rand,
		pub xres: Xres,
	}

	impl From<&AvImsGbaEapAka> for AvImsGbaEapAka {
		fn from(value: &AvImsGbaEapAka) -> Self {
			value.clone()
		}
	}

	/// AvType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "5G_HE_AKA",
	///    "EAP_AKA_PRIME"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AvType {
		#[default]
		#[serde(rename = "5G_HE_AKA")]
		FiveGHeAka,
		#[serde(rename = "EAP_AKA_PRIME")]
		EapAkaPrime,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AvType> for AvType {
		fn from(value: &AvType) -> Self {
			value.clone()
		}
	}

	impl ToString for AvType {
		fn to_string(&self) -> String {
			match *self {
				Self::FiveGHeAka => "5G_HE_AKA".to_string(),
				Self::EapAkaPrime => "EAP_AKA_PRIME".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AvType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"5G_HE_AKA" => Ok(Self::FiveGHeAka),
				"EAP_AKA_PRIME" => Ok(Self::EapAkaPrime),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AvType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AvType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AvType {
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
		pub guami_list: Vec<Guami>,
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

	/// ChangeOfSupiPeiAssociationReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "newPei"
	///  ],
	///  "properties": {
	///    "newPei": {
	///      "$ref": "#/components/schemas/Pei"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChangeOfSupiPeiAssociationReport {
		#[serde(rename = "newPei")]
		pub new_pei: Pei,
	}

	impl From<&ChangeOfSupiPeiAssociationReport> for ChangeOfSupiPeiAssociationReport {
		fn from(value: &ChangeOfSupiPeiAssociationReport) -> Self {
			value.clone()
		}
	}

	/// CkPrime
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
		NewUnchecked
	)]
	pub struct CkPrime(String);

	impl ::std::ops::Deref for CkPrime {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CkPrime> for String {
		fn from(value: CkPrime) -> Self {
			value.0
		}
	}

	impl From<&CkPrime> for CkPrime {
		fn from(value: &CkPrime) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CkPrime {
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

	impl ::std::convert::TryFrom<&str> for CkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CkPrime {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
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

	/// CmInfoReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "newCmInfoList"
	///  ],
	///  "properties": {
	///    "newCmInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CmInfo"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "oldCmInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CmInfo"
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
	pub struct CmInfoReport {
		#[serde(rename = "newCmInfoList")]
		pub new_cm_info_list: Vec<CmInfo>,
		#[serde(
			rename = "oldCmInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub old_cm_info_list: Vec<CmInfo>,
	}

	impl From<&CmInfoReport> for CmInfoReport {
		fn from(value: &CmInfoReport) -> Self {
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

	/// CnType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SINGLE_4G",
	///    "SINGLE_5G",
	///    "DUAL_4G5G"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum CnType {
		#[default]
		#[serde(rename = "SINGLE_4G")]
		Single4g,
		#[serde(rename = "SINGLE_5G")]
		Single5g,
		#[serde(rename = "DUAL_4G5G")]
		Dual4g5g,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CnType> for CnType {
		fn from(value: &CnType) -> Self {
			value.clone()
		}
	}

	impl ToString for CnType {
		fn to_string(&self) -> String {
			match *self {
				Self::Single4g => "SINGLE_4G".to_string(),
				Self::Single5g => "SINGLE_5G".to_string(),
				Self::Dual4g5g => "DUAL_4G5G".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CnType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SINGLE_4G" => Ok(Self::Single4g),
				"SINGLE_5G" => Ok(Self::Single5g),
				"DUAL_4G5G" => Ok(Self::Dual4g5g),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CnType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CnType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CnType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// CnTypeChangeReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "newCnType"
	///  ],
	///  "properties": {
	///    "newCnType": {
	///      "$ref": "#/components/schemas/CnType"
	///    },
	///    "oldCnType": {
	///      "$ref": "#/components/schemas/CnType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CnTypeChangeReport {
		#[serde(rename = "newCnType")]
		pub new_cn_type: CnType,
		#[serde(rename = "oldCnType", default, skip_serializing_if = "Option::is_none")]
		pub old_cn_type: Option<CnType>,
	}

	impl From<&CnTypeChangeReport> for CnTypeChangeReport {
		fn from(value: &CnTypeChangeReport) -> Self {
			value.clone()
		}
	}

	/// CodeWord
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

	/// CodeWordInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CODEWORD_CHECK_IN_UE",
	///    "CODEWORD_CHECK_IN_GMLC"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum CodeWordInd {
		#[default]
		#[serde(rename = "CODEWORD_CHECK_IN_UE")]
		CodewordCheckInUe,
		#[serde(rename = "CODEWORD_CHECK_IN_GMLC")]
		CodewordCheckInGmlc,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CodeWordInd> for CodeWordInd {
		fn from(value: &CodeWordInd) -> Self {
			value.clone()
		}
	}

	impl ToString for CodeWordInd {
		fn to_string(&self) -> String {
			match *self {
				Self::CodewordCheckInUe => "CODEWORD_CHECK_IN_UE".to_string(),
				Self::CodewordCheckInGmlc => "CODEWORD_CHECK_IN_GMLC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CodeWordInd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CODEWORD_CHECK_IN_UE" => Ok(Self::CodewordCheckInUe),
				"CODEWORD_CHECK_IN_GMLC" => Ok(Self::CodewordCheckInGmlc),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CodeWordInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CodeWordInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CodeWordInd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// CommunicationCharacteristics
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "ppActiveTime": {
	///      "$ref": "#/components/schemas/PpActiveTime"
	///    },
	///    "ppDlPacketCount": {
	///      "$ref": "#/components/schemas/PpDlPacketCount"
	///    },
	///    "ppDlPacketCountExt": {
	///      "$ref": "#/components/schemas/PpDlPacketCountExt"
	///    },
	///    "ppMaximumLatency": {
	///      "$ref": "#/components/schemas/PpMaximumLatency"
	///    },
	///    "ppMaximumResponseTime": {
	///      "$ref": "#/components/schemas/PpMaximumResponseTime"
	///    },
	///    "ppSubsRegTimer": {
	///      "$ref": "#/components/schemas/PpSubsRegTimer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CommunicationCharacteristics(pub Option<CommunicationCharacteristicsInner>);

	impl ::std::ops::Deref for CommunicationCharacteristics {
		type Target = Option<CommunicationCharacteristicsInner>;
		fn deref(&self) -> &Option<CommunicationCharacteristicsInner> {
			&self.0
		}
	}

	impl From<CommunicationCharacteristics> for Option<CommunicationCharacteristicsInner> {
		fn from(value: CommunicationCharacteristics) -> Self {
			value.0
		}
	}

	impl From<&CommunicationCharacteristics> for CommunicationCharacteristics {
		fn from(value: &CommunicationCharacteristics) -> Self {
			value.clone()
		}
	}

	impl From<Option<CommunicationCharacteristicsInner>> for CommunicationCharacteristics {
		fn from(value: Option<CommunicationCharacteristicsInner>) -> Self {
			Self(value)
		}
	}

	/// CommunicationCharacteristicsAf
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "maximumLatency": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "maximumResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "ppDlPacketCount": {
	///      "$ref": "#/components/schemas/PpDlPacketCount"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CommunicationCharacteristicsAf(pub Option<CommunicationCharacteristicsAfInner>);

	impl ::std::ops::Deref for CommunicationCharacteristicsAf {
		type Target = Option<CommunicationCharacteristicsAfInner>;
		fn deref(&self) -> &Option<CommunicationCharacteristicsAfInner> {
			&self.0
		}
	}

	impl From<CommunicationCharacteristicsAf> for Option<CommunicationCharacteristicsAfInner> {
		fn from(value: CommunicationCharacteristicsAf) -> Self {
			value.0
		}
	}

	impl From<&CommunicationCharacteristicsAf> for CommunicationCharacteristicsAf {
		fn from(value: &CommunicationCharacteristicsAf) -> Self {
			value.clone()
		}
	}

	impl From<Option<CommunicationCharacteristicsAfInner>> for CommunicationCharacteristicsAf {
		fn from(value: Option<CommunicationCharacteristicsAfInner>) -> Self {
			Self(value)
		}
	}

	/// CommunicationCharacteristicsAfInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "maximumLatency": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "maximumResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "ppDlPacketCount": {
	///      "$ref": "#/components/schemas/PpDlPacketCount"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CommunicationCharacteristicsAfInner {
		#[serde(
			rename = "maximumLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_latency: Option<DurationSec>,
		#[serde(
			rename = "maximumResponseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_response_time: Option<DurationSec>,
		#[serde(
			rename = "ppDlPacketCount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_dl_packet_count: Option<PpDlPacketCount>,
	}

	impl From<&CommunicationCharacteristicsAfInner> for CommunicationCharacteristicsAfInner {
		fn from(value: &CommunicationCharacteristicsAfInner) -> Self {
			value.clone()
		}
	}

	/// CommunicationCharacteristicsInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "ppActiveTime": {
	///      "$ref": "#/components/schemas/PpActiveTime"
	///    },
	///    "ppDlPacketCount": {
	///      "$ref": "#/components/schemas/PpDlPacketCount"
	///    },
	///    "ppDlPacketCountExt": {
	///      "$ref": "#/components/schemas/PpDlPacketCountExt"
	///    },
	///    "ppMaximumLatency": {
	///      "$ref": "#/components/schemas/PpMaximumLatency"
	///    },
	///    "ppMaximumResponseTime": {
	///      "$ref": "#/components/schemas/PpMaximumResponseTime"
	///    },
	///    "ppSubsRegTimer": {
	///      "$ref": "#/components/schemas/PpSubsRegTimer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CommunicationCharacteristicsInner {
		#[serde(
			rename = "ppActiveTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_active_time: Option<PpActiveTime>,
		#[serde(
			rename = "ppDlPacketCount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_dl_packet_count: Option<PpDlPacketCount>,
		#[serde(
			rename = "ppDlPacketCountExt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_dl_packet_count_ext: Option<PpDlPacketCountExt>,
		#[serde(
			rename = "ppMaximumLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_maximum_latency: Option<PpMaximumLatency>,
		#[serde(
			rename = "ppMaximumResponseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_maximum_response_time: Option<PpMaximumResponseTime>,
		#[serde(
			rename = "ppSubsRegTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pp_subs_reg_timer: Option<PpSubsRegTimer>,
	}

	impl From<&CommunicationCharacteristicsInner> for CommunicationCharacteristicsInner {
		fn from(value: &CommunicationCharacteristicsInner) -> Self {
			value.clone()
		}
	}

	/// ConfidentialityKey
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
		NewUnchecked
	)]
	pub struct ConfidentialityKey(String);

	impl ::std::ops::Deref for ConfidentialityKey {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ConfidentialityKey> for String {
		fn from(value: ConfidentialityKey) -> Self {
			value.0
		}
	}

	impl From<&ConfidentialityKey> for ConfidentialityKey {
		fn from(value: &ConfidentialityKey) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ConfidentialityKey {
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

	impl ::std::convert::TryFrom<&str> for ConfidentialityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ConfidentialityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ConfidentialityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ConfidentialityKey {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// ContextInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "origHeaders": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "requestHeaders": {
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
	pub struct ContextInfo {
		#[serde(rename = "origHeaders", default, skip_serializing_if = "Vec::is_empty")]
		pub orig_headers: Vec<String>,
		#[serde(
			rename = "requestHeaders",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub request_headers: Vec<String>,
	}

	impl From<&ContextInfo> for ContextInfo {
		fn from(value: &ContextInfo) -> Self {
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
		NewUnchecked
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
		NewUnchecked
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

	/// CreatedEeSubscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "eeSubscription"
	///  ],
	///  "properties": {
	///    "eeSubscription": {
	///      "$ref": "#/components/schemas/EeSubscription"
	///    },
	///    "epcStatusInd": {
	///      "type": "boolean"
	///    },
	///    "eventReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MonitoringReport"
	///      },
	///      "minItems": 1
	///    },
	///    "failedMoniConfigsEPC": {
	///      "description": "A map (list of key-value pairs where referenceId
	/// converted from integer to string serves as key; see clause 6.4.6.3.2) of
	/// FailedMonitoringConfiguration, the key value \"ALL\" may be used to
	/// identify a map entry which contains the failed cause of the EE
	/// subscription was not successful in EPC domain.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/FailedMonitoringConfiguration"
	///      }
	///    },
	///    "failedMonitoringConfigs": {
	///      "description": "A map (list of key-value pairs where referenceId
	/// converted from integer to string serves as key; see clause 6.4.6.3.2) of
	/// FailedMonitoringConfiguration",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/FailedMonitoringConfiguration"
	///      }
	///    },
	///    "numberOfUes": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "resetIds": {
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
	pub struct CreatedEeSubscription {
		#[serde(rename = "eeSubscription")]
		pub ee_subscription: EeSubscription,
		#[serde(
			rename = "epcStatusInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub epc_status_ind: Option<bool>,
		#[serde(
			rename = "eventReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub event_reports: Vec<MonitoringReport>,
		/// A map (list of key-value pairs where referenceId converted from
		/// integer to string serves as key; see clause 6.4.6.3.2) of
		/// FailedMonitoringConfiguration, the key value "ALL" may be used to
		/// identify a map entry which contains the failed cause of the EE
		/// subscription was not successful in EPC domain.
		#[serde(
			rename = "failedMoniConfigsEPC",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub failed_moni_configs_epc:
			::std::collections::HashMap<String, FailedMonitoringConfiguration>,
		/// A map (list of key-value pairs where referenceId converted from
		/// integer to string serves as key; see clause 6.4.6.3.2) of
		/// FailedMonitoringConfiguration
		#[serde(
			rename = "failedMonitoringConfigs",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub failed_monitoring_configs:
			::std::collections::HashMap<String, FailedMonitoringConfiguration>,
		#[serde(
			rename = "numberOfUes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub number_of_ues: Option<Uinteger>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
	}

	impl From<&CreatedEeSubscription> for CreatedEeSubscription {
		fn from(value: &CreatedEeSubscription) -> Self {
			value.clone()
		}
	}

	/// Contains identities representing those UEs potentially affected by a
	/// data-loss event at the UDR
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains identities representing those UEs potentially
	/// affected by a data-loss event at the UDR",
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
	///    "lastReplicationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
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
	pub struct DataRestorationNotification {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "lastReplicationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_replication_time: Option<DateTime>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "sNssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssai_list: Vec<Snssai>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
	}

	impl From<&DataRestorationNotification> for DataRestorationNotification {
		fn from(value: &DataRestorationNotification) -> Self {
			value.clone()
		}
	}

	/// DataSetName
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "AM",
	///    "SMF_SEL",
	///    "UEC_SMF",
	///    "UEC_SMSF",
	///    "SMS_SUB",
	///    "SM",
	///    "TRACE",
	///    "SMS_MNG",
	///    "LCS_PRIVACY",
	///    "LCS_MO",
	///    "UEC_AMF",
	///    "V2X",
	///    "LCS_BCA",
	///    "PROSE",
	///    "UC",
	///    "MBS"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum DataSetName {
		#[default]
		#[serde(rename = "AM")]
		Am,
		#[serde(rename = "SMF_SEL")]
		SmfSel,
		#[serde(rename = "UEC_SMF")]
		UecSmf,
		#[serde(rename = "UEC_SMSF")]
		UecSmsf,
		#[serde(rename = "SMS_SUB")]
		SmsSub,
		#[serde(rename = "SM")]
		Sm,
		#[serde(rename = "TRACE")]
		Trace,
		#[serde(rename = "SMS_MNG")]
		SmsMng,
		#[serde(rename = "LCS_PRIVACY")]
		LcsPrivacy,
		#[serde(rename = "LCS_MO")]
		LcsMo,
		#[serde(rename = "UEC_AMF")]
		UecAmf,
		#[serde(rename = "V2X")]
		V2x,
		#[serde(rename = "LCS_BCA")]
		LcsBca,
		#[serde(rename = "PROSE")]
		Prose,
		#[serde(rename = "UC")]
		Uc,
		#[serde(rename = "MBS")]
		Mbs,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DataSetName> for DataSetName {
		fn from(value: &DataSetName) -> Self {
			value.clone()
		}
	}

	impl ToString for DataSetName {
		fn to_string(&self) -> String {
			match *self {
				Self::Am => "AM".to_string(),
				Self::SmfSel => "SMF_SEL".to_string(),
				Self::UecSmf => "UEC_SMF".to_string(),
				Self::UecSmsf => "UEC_SMSF".to_string(),
				Self::SmsSub => "SMS_SUB".to_string(),
				Self::Sm => "SM".to_string(),
				Self::Trace => "TRACE".to_string(),
				Self::SmsMng => "SMS_MNG".to_string(),
				Self::LcsPrivacy => "LCS_PRIVACY".to_string(),
				Self::LcsMo => "LCS_MO".to_string(),
				Self::UecAmf => "UEC_AMF".to_string(),
				Self::V2x => "V2X".to_string(),
				Self::LcsBca => "LCS_BCA".to_string(),
				Self::Prose => "PROSE".to_string(),
				Self::Uc => "UC".to_string(),
				Self::Mbs => "MBS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DataSetName {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AM" => Ok(Self::Am),
				"SMF_SEL" => Ok(Self::SmfSel),
				"UEC_SMF" => Ok(Self::UecSmf),
				"UEC_SMSF" => Ok(Self::UecSmsf),
				"SMS_SUB" => Ok(Self::SmsSub),
				"SM" => Ok(Self::Sm),
				"TRACE" => Ok(Self::Trace),
				"SMS_MNG" => Ok(Self::SmsMng),
				"LCS_PRIVACY" => Ok(Self::LcsPrivacy),
				"LCS_MO" => Ok(Self::LcsMo),
				"UEC_AMF" => Ok(Self::UecAmf),
				"V2X" => Ok(Self::V2x),
				"LCS_BCA" => Ok(Self::LcsBca),
				"PROSE" => Ok(Self::Prose),
				"UC" => Ok(Self::Uc),
				"MBS" => Ok(Self::Mbs),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// DatalinkReportingConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "dddStatusList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
	///      },
	///      "minItems": 1
	///    },
	///    "dddTrafficDes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "slice": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DatalinkReportingConfiguration {
		#[serde(
			rename = "dddStatusList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_status_list: Vec<DlDataDeliveryStatus>,
		#[serde(
			rename = "dddTrafficDes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_traffic_des: Vec<DddTrafficDescriptor>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub slice: Option<Snssai>,
	}

	impl From<&DatalinkReportingConfiguration> for DatalinkReportingConfiguration {
		fn from(value: &DatalinkReportingConfiguration) -> Self {
			value.clone()
		}
	}

	/// DatasetNames
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/DataSetName"
	///  },
	///  "minItems": 2,
	///  "uniqueItems": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DatasetNames(pub Vec<DataSetName>);

	impl ::std::ops::Deref for DatasetNames {
		type Target = Vec<DataSetName>;
		fn deref(&self) -> &Vec<DataSetName> {
			&self.0
		}
	}

	impl From<DatasetNames> for Vec<DataSetName> {
		fn from(value: DatasetNames) -> Self {
			value.0
		}
	}

	impl From<&DatasetNames> for DatasetNames {
		fn from(value: &DatasetNames) -> Self {
			value.clone()
		}
	}

	impl From<Vec<DataSetName>> for DatasetNames {
		fn from(value: Vec<DataSetName>) -> Self {
			Self(value)
		}
	}

	/// Deconceal Request Data
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Deconceal Request Data",
	///  "type": "object",
	///  "required": [
	///    "suci"
	///  ],
	///  "properties": {
	///    "suci": {
	///      "$ref": "#/components/schemas/Suci"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DeconcealReqData {
		pub suci: Suci,
	}

	impl From<&DeconcealReqData> for DeconcealReqData {
		fn from(value: &DeconcealReqData) -> Self {
			value.clone()
		}
	}

	/// Deconceal Response Data
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Deconceal Response Data",
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
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
	pub struct DeconcealRspData {
		pub supi: Supi,
	}

	impl From<&DeconcealRspData> for DeconcealRspData {
		fn from(value: &DeconcealRspData) -> Self {
			value.clone()
		}
	}

	/// DefaultUnrelatedClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "allowedGeographicArea": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 1
	///    },
	///    "codeWordInd": {
	///      "$ref": "#/components/schemas/CodeWordInd"
	///    },
	///    "codeWordList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodeWord"
	///      },
	///      "minItems": 1
	///    },
	///    "privacyCheckRelatedAction": {
	///      "$ref": "#/components/schemas/PrivacyCheckRelatedAction"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DefaultUnrelatedClass {
		#[serde(
			rename = "allowedGeographicArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_geographic_area: Vec<GeographicArea>,
		#[serde(
			rename = "codeWordInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub code_word_ind: Option<CodeWordInd>,
		#[serde(
			rename = "codeWordList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub code_word_list: Vec<CodeWord>,
		#[serde(
			rename = "privacyCheckRelatedAction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub privacy_check_related_action: Option<PrivacyCheckRelatedAction>,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&DefaultUnrelatedClass> for DefaultUnrelatedClass {
		fn from(value: &DefaultUnrelatedClass) -> Self {
			value.clone()
		}
	}

	/// DeregistrationData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "deregReason"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "deregReason": {
	///      "$ref": "#/components/schemas/DeregistrationReason"
	///    },
	///    "newSmfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
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
	pub struct DeregistrationData {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(rename = "deregReason")]
		pub dereg_reason: DeregistrationReason,
		#[serde(
			rename = "newSmfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub new_smf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "pduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_id: Option<PduSessionId>,
	}

	impl From<&DeregistrationData> for DeregistrationData {
		fn from(value: &DeregistrationData) -> Self {
			value.clone()
		}
	}

	/// DeregistrationReason
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "UE_INITIAL_REGISTRATION",
	///    "UE_REGISTRATION_AREA_CHANGE",
	///    "SUBSCRIPTION_WITHDRAWN",
	///    "5GS_TO_EPS_MOBILITY",
	///    "5GS_TO_EPS_MOBILITY_UE_INITIAL_REGISTRATION",
	///    "REREGISTRATION_REQUIRED",
	///    "SMF_CONTEXT_TRANSFERRED",
	///    "DUPLICATE_PDU_SESSION",
	///    "DISASTER_CONDITION_TERMINATED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum DeregistrationReason {
		#[default]
		#[serde(rename = "UE_INITIAL_REGISTRATION")]
		UeInitialRegistration,
		#[serde(rename = "UE_REGISTRATION_AREA_CHANGE")]
		UeRegistrationAreaChange,
		#[serde(rename = "SUBSCRIPTION_WITHDRAWN")]
		SubscriptionWithdrawn,
		#[serde(rename = "5GS_TO_EPS_MOBILITY")]
		FiveGsToEpsMobility,
		#[serde(rename = "5GS_TO_EPS_MOBILITY_UE_INITIAL_REGISTRATION")]
		FiveGsToEpsMobilityUeInitialRegistration,
		#[serde(rename = "REREGISTRATION_REQUIRED")]
		ReregistrationRequired,
		#[serde(rename = "SMF_CONTEXT_TRANSFERRED")]
		SmfContextTransferred,
		#[serde(rename = "DUPLICATE_PDU_SESSION")]
		DuplicatePduSession,
		#[serde(rename = "DISASTER_CONDITION_TERMINATED")]
		DisasterConditionTerminated,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DeregistrationReason> for DeregistrationReason {
		fn from(value: &DeregistrationReason) -> Self {
			value.clone()
		}
	}

	impl ToString for DeregistrationReason {
		fn to_string(&self) -> String {
			match *self {
				Self::UeInitialRegistration => "UE_INITIAL_REGISTRATION".to_string(),
				Self::UeRegistrationAreaChange => "UE_REGISTRATION_AREA_CHANGE".to_string(),
				Self::SubscriptionWithdrawn => "SUBSCRIPTION_WITHDRAWN".to_string(),
				Self::FiveGsToEpsMobility => "5GS_TO_EPS_MOBILITY".to_string(),
				Self::FiveGsToEpsMobilityUeInitialRegistration => {
					"5GS_TO_EPS_MOBILITY_UE_INITIAL_REGISTRATION".to_string()
				}
				Self::ReregistrationRequired => "REREGISTRATION_REQUIRED".to_string(),
				Self::SmfContextTransferred => "SMF_CONTEXT_TRANSFERRED".to_string(),
				Self::DuplicatePduSession => "DUPLICATE_PDU_SESSION".to_string(),
				Self::DisasterConditionTerminated => "DISASTER_CONDITION_TERMINATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DeregistrationReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_INITIAL_REGISTRATION" => Ok(Self::UeInitialRegistration),
				"UE_REGISTRATION_AREA_CHANGE" => Ok(Self::UeRegistrationAreaChange),
				"SUBSCRIPTION_WITHDRAWN" => Ok(Self::SubscriptionWithdrawn),
				"5GS_TO_EPS_MOBILITY" => Ok(Self::FiveGsToEpsMobility),
				"5GS_TO_EPS_MOBILITY_UE_INITIAL_REGISTRATION" => {
					Ok(Self::FiveGsToEpsMobilityUeInitialRegistration)
				}
				"REREGISTRATION_REQUIRED" => Ok(Self::ReregistrationRequired),
				"SMF_CONTEXT_TRANSFERRED" => Ok(Self::SmfContextTransferred),
				"DUPLICATE_PDU_SESSION" => Ok(Self::DuplicatePduSession),
				"DISASTER_CONDITION_TERMINATED" => Ok(Self::DisasterConditionTerminated),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DeregistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DeregistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DeregistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// DnnConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "pduSessionTypes",
	///    "sscModes"
	///  ],
	///  "properties": {
	///    "3gppChargingCharacteristics": {
	///      "$ref": "#/components/schemas/3GppChargingCharacteristics"
	///    },
	///    "5gQosProfile": {
	///      "$ref": "#/components/schemas/SubscribedDefaultQos"
	///    },
	///    "acsInfo": {
	///      "$ref": "#/components/schemas/AcsInfo"
	///    },
	///    "additionalDnAaaAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddress"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalEcsAddrConfigInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalSharedEcsAddrConfigInfoIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataId"
	///      },
	///      "minItems": 1
	///    },
	///    "aerialUeInd": {
	///      "$ref": "#/components/schemas/AerialUeIndication"
	///    },
	///    "atsssAllowed": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "dnAaaAddress": {
	///      "$ref": "#/components/schemas/IpAddress"
	///    },
	///    "dnAaaFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "dnAaaIpAddressAllocation": {
	///      "type": "boolean"
	///    },
	///    "easDiscoveryAuthorized": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecsAddrConfigInfo": {
	///      "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///    },
	///    "iptvAccCtrlInfo": {
	///      "type": "string"
	///    },
	///    "ipv4FrameRouteList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FrameRouteInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "ipv6FrameRouteList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FrameRouteInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "iwkEpsInd": {
	///      "$ref": "#/components/schemas/IwkEpsInd"
	///    },
	///    "niddInfo": {
	///      "$ref": "#/components/schemas/NiddInformation"
	///    },
	///    "niddNefId": {
	///      "$ref": "#/components/schemas/NefId"
	///    },
	///    "onboardingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pduSessionContinuityInd": {
	///      "$ref": "#/components/schemas/PduSessionContinuityInd"
	///    },
	///    "pduSessionTypes": {
	///      "$ref": "#/components/schemas/PduSessionTypes"
	///    },
	///    "redundantSessionAllowed": {
	///      "type": "boolean"
	///    },
	///    "secondaryAuth": {
	///      "type": "boolean"
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "sharedEcsAddrConfigInfo": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "sscModes": {
	///      "$ref": "#/components/schemas/SscModes"
	///    },
	///    "staticIpAddress": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddress"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "subscribedMaxIpv6PrefixSize": {
	///      "type": "integer"
	///    },
	///    "uavSecondaryAuth": {
	///      "default": false,
	///      "type": "boolean"
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
	pub struct DnnConfiguration {
		#[serde(rename = "acsInfo", default, skip_serializing_if = "Option::is_none")]
		pub acs_info: Option<AcsInfo>,
		#[serde(
			rename = "additionalDnAaaAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_dn_aaa_addresses: Vec<IpAddress>,
		#[serde(
			rename = "additionalEcsAddrConfigInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_ecs_addr_config_infos: Vec<EcsAddrConfigInfo>,
		#[serde(
			rename = "additionalSharedEcsAddrConfigInfoIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_shared_ecs_addr_config_info_ids: Vec<SharedDataId>,
		#[serde(
			rename = "aerialUeInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aerial_ue_ind: Option<AerialUeIndication>,
		#[serde(rename = "atsssAllowed", default)]
		pub atsss_allowed: bool,
		#[serde(
			rename = "dnAaaAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_aaa_address: Option<IpAddress>,
		#[serde(rename = "dnAaaFqdn", default, skip_serializing_if = "Option::is_none")]
		pub dn_aaa_fqdn: Option<Fqdn>,
		#[serde(
			rename = "dnAaaIpAddressAllocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_aaa_ip_address_allocation: Option<bool>,
		#[serde(rename = "easDiscoveryAuthorized", default)]
		pub eas_discovery_authorized: bool,
		#[serde(
			rename = "ecsAddrConfigInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ecs_addr_config_info: Option<EcsAddrConfigInfo>,
		#[serde(
			rename = "5gQosProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_qos_profile: Option<SubscribedDefaultQos>,
		#[serde(
			rename = "iptvAccCtrlInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub iptv_acc_ctrl_info: Option<String>,
		#[serde(
			rename = "ipv4FrameRouteList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_frame_route_list: Vec<FrameRouteInfo>,
		#[serde(rename = "ipv4Index", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_index: Option<IpIndex>,
		#[serde(
			rename = "ipv6FrameRouteList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_frame_route_list: Vec<FrameRouteInfo>,
		#[serde(rename = "ipv6Index", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_index: Option<IpIndex>,
		#[serde(rename = "iwkEpsInd", default, skip_serializing_if = "Option::is_none")]
		pub iwk_eps_ind: Option<IwkEpsInd>,
		#[serde(rename = "niddInfo", default, skip_serializing_if = "Option::is_none")]
		pub nidd_info: Option<NiddInformation>,
		#[serde(rename = "niddNefId", default, skip_serializing_if = "Option::is_none")]
		pub nidd_nef_id: Option<NefId>,
		#[serde(rename = "onboardingInd", default)]
		pub onboarding_ind: bool,
		#[serde(
			rename = "pduSessionContinuityInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_continuity_ind: Option<PduSessionContinuityInd>,
		#[serde(rename = "pduSessionTypes")]
		pub pdu_session_types: PduSessionTypes,
		#[serde(
			rename = "redundantSessionAllowed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redundant_session_allowed: Option<bool>,
		#[serde(
			rename = "secondaryAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_auth: Option<bool>,
		#[serde(
			rename = "sessionAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_ambr: Option<Ambr>,
		#[serde(
			rename = "sharedEcsAddrConfigInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_ecs_addr_config_info: Option<SharedDataId>,
		#[serde(rename = "sscModes")]
		pub ssc_modes: SscModes,
		#[serde(
			rename = "staticIpAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub static_ip_address: Vec<IpAddress>,
		#[serde(
			rename = "subscribedMaxIpv6PrefixSize",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribed_max_ipv6_prefix_size: Option<i64>,
		#[serde(
			rename = "3gppChargingCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_charging_characteristics: Option<_3gppChargingCharacteristics>,
		#[serde(rename = "uavSecondaryAuth", default)]
		pub uav_secondary_auth: bool,
		#[serde(
			rename = "upSecurity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_security: Option<UpSecurity>,
	}

	impl From<&DnnConfiguration> for DnnConfiguration {
		fn from(value: &DnnConfiguration) -> Self {
			value.clone()
		}
	}

	/// DnnIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnIndicator(pub bool);

	impl ::std::ops::Deref for DnnIndicator {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<DnnIndicator> for bool {
		fn from(value: DnnIndicator) -> Self {
			value.0
		}
	}

	impl From<&DnnIndicator> for DnnIndicator {
		fn from(value: &DnnIndicator) -> Self {
			value.clone()
		}
	}

	impl From<bool> for DnnIndicator {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DnnIndicator {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DnnIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DnnIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DnnIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DnnIndicator {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// DnnInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "defaultDnnIndicator": {
	///      "$ref": "#/components/schemas/DnnIndicator"
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
	///    },
	///    "dnnBarred": {
	///      "type": "boolean"
	///    },
	///    "invokeNefInd": {
	///      "type": "boolean"
	///    },
	///    "iwkEpsInd": {
	///      "$ref": "#/components/schemas/IwkEpsInd"
	///    },
	///    "lboRoamingAllowed": {
	///      "$ref": "#/components/schemas/LboRoamingAllowed"
	///    },
	///    "sameSmfInd": {
	///      "type": "boolean"
	///    },
	///    "smfList": {
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
	pub struct DnnInfo {
		#[serde(
			rename = "defaultDnnIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub default_dnn_indicator: Option<DnnIndicator>,
		pub dnn: DnnInfoDnn,
		#[serde(rename = "dnnBarred", default, skip_serializing_if = "Option::is_none")]
		pub dnn_barred: Option<bool>,
		#[serde(
			rename = "invokeNefInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invoke_nef_ind: Option<bool>,
		#[serde(rename = "iwkEpsInd", default, skip_serializing_if = "Option::is_none")]
		pub iwk_eps_ind: Option<IwkEpsInd>,
		#[serde(
			rename = "lboRoamingAllowed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lbo_roaming_allowed: Option<LboRoamingAllowed>,
		#[serde(
			rename = "sameSmfInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub same_smf_ind: Option<bool>,
		#[serde(rename = "smfList", default, skip_serializing_if = "Vec::is_empty")]
		pub smf_list: Vec<NfInstanceId>,
	}

	impl From<&DnnInfo> for DnnInfo {
		fn from(value: &DnnInfo) -> Self {
			value.clone()
		}
	}

	/// DnnInfoDnn
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
	pub struct DnnInfoDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnInfoDnn> for DnnInfoDnn {
		fn from(value: &DnnInfoDnn) -> Self {
			value.clone()
		}
	}

	/// DualRegistrationFlag
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DualRegistrationFlag(pub bool);

	impl ::std::ops::Deref for DualRegistrationFlag {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<DualRegistrationFlag> for bool {
		fn from(value: DualRegistrationFlag) -> Self {
			value.0
		}
	}

	impl From<&DualRegistrationFlag> for DualRegistrationFlag {
		fn from(value: &DualRegistrationFlag) -> Self {
			value.clone()
		}
	}

	impl From<bool> for DualRegistrationFlag {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DualRegistrationFlag {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DualRegistrationFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DualRegistrationFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DualRegistrationFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DualRegistrationFlag {
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
		NewUnchecked
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

	/// EcRestriction
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "plmnEcInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnEcInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EcRestriction(pub Option<EcRestrictionInner>);

	impl ::std::ops::Deref for EcRestriction {
		type Target = Option<EcRestrictionInner>;
		fn deref(&self) -> &Option<EcRestrictionInner> {
			&self.0
		}
	}

	impl From<EcRestriction> for Option<EcRestrictionInner> {
		fn from(value: EcRestriction) -> Self {
			value.0
		}
	}

	impl From<&EcRestriction> for EcRestriction {
		fn from(value: &EcRestriction) -> Self {
			value.clone()
		}
	}

	impl From<Option<EcRestrictionInner>> for EcRestriction {
		fn from(value: Option<EcRestrictionInner>) -> Self {
			Self(value)
		}
	}

	/// EcRestrictionDataWb
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ecModeARestricted"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ecModeBRestricted"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ecModeARestricted": {
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
	#[serde(untagged)]
	pub enum EcRestrictionDataWb {
		#[default]
		Variant0 {
			#[serde(rename = "ecModeARestricted")]
			ec_mode_a_restricted: bool,
		},
		Variant1 {
			#[serde(rename = "ecModeBRestricted")]
			ec_mode_b_restricted: bool,
		},
	}

	impl From<&EcRestrictionDataWb> for EcRestrictionDataWb {
		fn from(value: &EcRestrictionDataWb) -> Self {
			value.clone()
		}
	}

	/// EcRestrictionInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "plmnEcInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnEcInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EcRestrictionInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "plmnEcInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub plmn_ec_infos: Vec<PlmnEcInfo>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
	}

	impl From<&EcRestrictionInner> for EcRestrictionInner {
		fn from(value: &EcRestrictionInner) -> Self {
			value.clone()
		}
	}

	/// EcsAddrConfigInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ecsServerAddr": {
	///      "$ref": "#/components/schemas/EcsServerAddr"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "spatialValidityCond": {
	///      "$ref": "#/components/schemas/SpatialValidityCond"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EcsAddrConfigInfo(pub Option<EcsAddrConfigInfoInner>);

	impl ::std::ops::Deref for EcsAddrConfigInfo {
		type Target = Option<EcsAddrConfigInfoInner>;
		fn deref(&self) -> &Option<EcsAddrConfigInfoInner> {
			&self.0
		}
	}

	impl From<EcsAddrConfigInfo> for Option<EcsAddrConfigInfoInner> {
		fn from(value: EcsAddrConfigInfo) -> Self {
			value.0
		}
	}

	impl From<&EcsAddrConfigInfo> for EcsAddrConfigInfo {
		fn from(value: &EcsAddrConfigInfo) -> Self {
			value.clone()
		}
	}

	impl From<Option<EcsAddrConfigInfoInner>> for EcsAddrConfigInfo {
		fn from(value: Option<EcsAddrConfigInfoInner>) -> Self {
			Self(value)
		}
	}

	/// EcsAddrConfigInfoInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ecsServerAddr": {
	///      "$ref": "#/components/schemas/EcsServerAddr"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "spatialValidityCond": {
	///      "$ref": "#/components/schemas/SpatialValidityCond"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EcsAddrConfigInfoInner {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "ecsServerAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ecs_server_addr: Option<EcsServerAddr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(
			rename = "spatialValidityCond",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub spatial_validity_cond: Option<SpatialValidityCond>,
	}

	impl From<&EcsAddrConfigInfoInner> for EcsAddrConfigInfoInner {
		fn from(value: &EcsAddrConfigInfoInner) -> Self {
			value.clone()
		}
	}

	/// EdrxParameters
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "edrxValue",
	///    "ratType"
	///  ],
	///  "properties": {
	///    "edrxValue": {
	///      "type": "string",
	///      "pattern": "^([0-1]{4})$"
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
	pub struct EdrxParameters {
		#[serde(rename = "edrxValue")]
		pub edrx_value: EdrxParametersEdrxValue,
		#[serde(rename = "ratType")]
		pub rat_type: RatType,
	}

	impl From<&EdrxParameters> for EdrxParameters {
		fn from(value: &EdrxParameters) -> Self {
			value.clone()
		}
	}

	/// EdrxParametersEdrxValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([0-1]{4})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct EdrxParametersEdrxValue(String);

	impl ::std::ops::Deref for EdrxParametersEdrxValue {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EdrxParametersEdrxValue> for String {
		fn from(value: EdrxParametersEdrxValue) -> Self {
			value.0
		}
	}

	impl From<&EdrxParametersEdrxValue> for EdrxParametersEdrxValue {
		fn from(value: &EdrxParametersEdrxValue) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EdrxParametersEdrxValue {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-1]{4})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([0-1]{4})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for EdrxParametersEdrxValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EdrxParametersEdrxValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EdrxParametersEdrxValue {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EdrxParametersEdrxValue {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// EeMonitoringRevoked
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "revokedMonitoringEventList"
	///  ],
	///  "properties": {
	///    "excludeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "removedGpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "revokedMonitoringEventList": {
	///      "description": "A map (list of key-value pairs where ReferenceId
	/// serves as key) of MonitoringEvents",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MonitoringEvent"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EeMonitoringRevoked {
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "removedGpsi",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub removed_gpsi: Option<Gpsi>,
		/// A map (list of key-value pairs where ReferenceId serves as key) of
		/// MonitoringEvents
		#[serde(rename = "revokedMonitoringEventList")]
		pub revoked_monitoring_event_list: ::std::collections::HashMap<String, MonitoringEvent>,
	}

	impl From<&EeMonitoringRevoked> for EeMonitoringRevoked {
		fn from(value: &EeMonitoringRevoked) -> Self {
			value.clone()
		}
	}

	/// EeSubscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "callbackReference",
	///    "monitoringConfigurations"
	///  ],
	///  "properties": {
	///    "callbackReference": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "epcAppliedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "excludeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "includeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "monitoringConfigurations": {
	///      "description": "A map (list of key-value pairs where ReferenceId
	/// serves as key) of MonitoringConfigurations",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MonitoringConfiguration"
	///      }
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "reportingOptions": {
	///      "$ref": "#/components/schemas/ReportingOptions"
	///    },
	///    "scefDiamHost": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "scefDiamRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "secondCallbackRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "subscriptionId": {
	///      "type": "string"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
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
	pub struct EeSubscription {
		#[serde(rename = "callbackReference")]
		pub callback_reference: Uri,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(rename = "epcAppliedInd", default)]
		pub epc_applied_ind: bool,
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "includeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_gpsi_list: Vec<Gpsi>,
		/// A map (list of key-value pairs where ReferenceId serves as key) of
		/// MonitoringConfigurations
		#[serde(rename = "monitoringConfigurations")]
		pub monitoring_configurations: ::std::collections::HashMap<String, MonitoringConfiguration>,
		#[serde(
			rename = "notifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_correlation_id: Option<String>,
		#[serde(
			rename = "reportingOptions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_options: Option<ReportingOptions>,
		#[serde(
			rename = "scefDiamHost",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scef_diam_host: Option<Fqdn>,
		#[serde(
			rename = "scefDiamRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scef_diam_realm: Option<Fqdn>,
		#[serde(
			rename = "secondCallbackRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub second_callback_ref: Option<Uri>,
		#[serde(
			rename = "subscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscription_id: Option<String>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
	}

	impl From<&EeSubscription> for EeSubscription {
		fn from(value: &EeSubscription) -> Self {
			value.clone()
		}
	}

	/// EmergencyInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "pgwFqdn"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "pgwIpAddress"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "epdgInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "pgwIpAddress": {
	///      "$ref": "#/components/schemas/IpAddress"
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
	#[serde(untagged)]
	pub enum EmergencyInfo {
		#[default]
		Variant0 {
			#[serde(rename = "epdgInd", default)]
			epdg_ind: bool,
			#[serde(rename = "pgwFqdn")]
			pgw_fqdn: Fqdn,
			#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
			plmn_id: Option<PlmnId>,
			#[serde(
				rename = "smfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			smf_instance_id: Option<NfInstanceId>,
		},
		Variant1 {
			#[serde(rename = "epdgInd", default)]
			epdg_ind: bool,
			#[serde(rename = "pgwIpAddress")]
			pgw_ip_address: IpAddress,
			#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
			plmn_id: Option<PlmnId>,
			#[serde(
				rename = "smfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			smf_instance_id: Option<NfInstanceId>,
		},
	}

	impl From<&EmergencyInfo> for EmergencyInfo {
		fn from(value: &EmergencyInfo) -> Self {
			value.clone()
		}
	}

	/// EnhancedCoverageRestrictionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "plmnEcInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnEcInfo"
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
	pub struct EnhancedCoverageRestrictionData {
		#[serde(
			rename = "plmnEcInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub plmn_ec_info_list: Vec<PlmnEcInfo>,
	}

	impl From<&EnhancedCoverageRestrictionData> for EnhancedCoverageRestrictionData {
		fn from(value: &EnhancedCoverageRestrictionData) -> Self {
			value.clone()
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

	/// EventReportMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "PERIODIC",
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
	pub enum EventReportMode {
		#[default]
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(rename = "ON_EVENT_DETECTION")]
		OnEventDetection,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EventReportMode> for EventReportMode {
		fn from(value: &EventReportMode) -> Self {
			value.clone()
		}
	}

	impl ToString for EventReportMode {
		fn to_string(&self) -> String {
			match *self {
				Self::Periodic => "PERIODIC".to_string(),
				Self::OnEventDetection => "ON_EVENT_DETECTION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EventReportMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERIODIC" => Ok(Self::Periodic),
				"ON_EVENT_DETECTION" => Ok(Self::OnEventDetection),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EventReportMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EventReportMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EventReportMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// EventType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "LOSS_OF_CONNECTIVITY",
	///    "UE_REACHABILITY_FOR_DATA",
	///    "UE_REACHABILITY_FOR_SMS",
	///    "LOCATION_REPORTING",
	///    "CHANGE_OF_SUPI_PEI_ASSOCIATION",
	///    "ROAMING_STATUS",
	///    "COMMUNICATION_FAILURE",
	///    "AVAILABILITY_AFTER_DDN_FAILURE",
	///    "CN_TYPE_CHANGE",
	///    "DL_DATA_DELIVERY_STATUS",
	///    "PDN_CONNECTIVITY_STATUS",
	///    "UE_CONNECTION_MANAGEMENT_STATE",
	///    "ACCESS_TYPE_REPORT",
	///    "REGISTRATION_STATE_REPORT",
	///    "CONNECTIVITY_STATE_REPORT",
	///    "TYPE_ALLOCATION_CODE_REPORT",
	///    "FREQUENT_MOBILITY_REGISTRATION_REPORT",
	///    "PDU_SES_REL",
	///    "PDU_SES_EST",
	///    "UE_MEMORY_AVAILABLE_FOR_SMS"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum EventType {
		#[default]
		#[serde(rename = "LOSS_OF_CONNECTIVITY")]
		LossOfConnectivity,
		#[serde(rename = "UE_REACHABILITY_FOR_DATA")]
		UeReachabilityForData,
		#[serde(rename = "UE_REACHABILITY_FOR_SMS")]
		UeReachabilityForSms,
		#[serde(rename = "LOCATION_REPORTING")]
		LocationReporting,
		#[serde(rename = "CHANGE_OF_SUPI_PEI_ASSOCIATION")]
		ChangeOfSupiPeiAssociation,
		#[serde(rename = "ROAMING_STATUS")]
		RoamingStatus,
		#[serde(rename = "COMMUNICATION_FAILURE")]
		CommunicationFailure,
		#[serde(rename = "AVAILABILITY_AFTER_DDN_FAILURE")]
		AvailabilityAfterDdnFailure,
		#[serde(rename = "CN_TYPE_CHANGE")]
		CnTypeChange,
		#[serde(rename = "DL_DATA_DELIVERY_STATUS")]
		DlDataDeliveryStatus,
		#[serde(rename = "PDN_CONNECTIVITY_STATUS")]
		PdnConnectivityStatus,
		#[serde(rename = "UE_CONNECTION_MANAGEMENT_STATE")]
		UeConnectionManagementState,
		#[serde(rename = "ACCESS_TYPE_REPORT")]
		AccessTypeReport,
		#[serde(rename = "REGISTRATION_STATE_REPORT")]
		RegistrationStateReport,
		#[serde(rename = "CONNECTIVITY_STATE_REPORT")]
		ConnectivityStateReport,
		#[serde(rename = "TYPE_ALLOCATION_CODE_REPORT")]
		TypeAllocationCodeReport,
		#[serde(rename = "FREQUENT_MOBILITY_REGISTRATION_REPORT")]
		FrequentMobilityRegistrationReport,
		#[serde(rename = "PDU_SES_REL")]
		PduSesRel,
		#[serde(rename = "PDU_SES_EST")]
		PduSesEst,
		#[serde(rename = "UE_MEMORY_AVAILABLE_FOR_SMS")]
		UeMemoryAvailableForSms,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EventType> for EventType {
		fn from(value: &EventType) -> Self {
			value.clone()
		}
	}

	impl ToString for EventType {
		fn to_string(&self) -> String {
			match *self {
				Self::LossOfConnectivity => "LOSS_OF_CONNECTIVITY".to_string(),
				Self::UeReachabilityForData => "UE_REACHABILITY_FOR_DATA".to_string(),
				Self::UeReachabilityForSms => "UE_REACHABILITY_FOR_SMS".to_string(),
				Self::LocationReporting => "LOCATION_REPORTING".to_string(),
				Self::ChangeOfSupiPeiAssociation => "CHANGE_OF_SUPI_PEI_ASSOCIATION".to_string(),
				Self::RoamingStatus => "ROAMING_STATUS".to_string(),
				Self::CommunicationFailure => "COMMUNICATION_FAILURE".to_string(),
				Self::AvailabilityAfterDdnFailure => "AVAILABILITY_AFTER_DDN_FAILURE".to_string(),
				Self::CnTypeChange => "CN_TYPE_CHANGE".to_string(),
				Self::DlDataDeliveryStatus => "DL_DATA_DELIVERY_STATUS".to_string(),
				Self::PdnConnectivityStatus => "PDN_CONNECTIVITY_STATUS".to_string(),
				Self::UeConnectionManagementState => "UE_CONNECTION_MANAGEMENT_STATE".to_string(),
				Self::AccessTypeReport => "ACCESS_TYPE_REPORT".to_string(),
				Self::RegistrationStateReport => "REGISTRATION_STATE_REPORT".to_string(),
				Self::ConnectivityStateReport => "CONNECTIVITY_STATE_REPORT".to_string(),
				Self::TypeAllocationCodeReport => "TYPE_ALLOCATION_CODE_REPORT".to_string(),
				Self::FrequentMobilityRegistrationReport => {
					"FREQUENT_MOBILITY_REGISTRATION_REPORT".to_string()
				}
				Self::PduSesRel => "PDU_SES_REL".to_string(),
				Self::PduSesEst => "PDU_SES_EST".to_string(),
				Self::UeMemoryAvailableForSms => "UE_MEMORY_AVAILABLE_FOR_SMS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOSS_OF_CONNECTIVITY" => Ok(Self::LossOfConnectivity),
				"UE_REACHABILITY_FOR_DATA" => Ok(Self::UeReachabilityForData),
				"UE_REACHABILITY_FOR_SMS" => Ok(Self::UeReachabilityForSms),
				"LOCATION_REPORTING" => Ok(Self::LocationReporting),
				"CHANGE_OF_SUPI_PEI_ASSOCIATION" => Ok(Self::ChangeOfSupiPeiAssociation),
				"ROAMING_STATUS" => Ok(Self::RoamingStatus),
				"COMMUNICATION_FAILURE" => Ok(Self::CommunicationFailure),
				"AVAILABILITY_AFTER_DDN_FAILURE" => Ok(Self::AvailabilityAfterDdnFailure),
				"CN_TYPE_CHANGE" => Ok(Self::CnTypeChange),
				"DL_DATA_DELIVERY_STATUS" => Ok(Self::DlDataDeliveryStatus),
				"PDN_CONNECTIVITY_STATUS" => Ok(Self::PdnConnectivityStatus),
				"UE_CONNECTION_MANAGEMENT_STATE" => Ok(Self::UeConnectionManagementState),
				"ACCESS_TYPE_REPORT" => Ok(Self::AccessTypeReport),
				"REGISTRATION_STATE_REPORT" => Ok(Self::RegistrationStateReport),
				"CONNECTIVITY_STATE_REPORT" => Ok(Self::ConnectivityStateReport),
				"TYPE_ALLOCATION_CODE_REPORT" => Ok(Self::TypeAllocationCodeReport),
				"FREQUENT_MOBILITY_REGISTRATION_REPORT" => {
					Ok(Self::FrequentMobilityRegistrationReport)
				}
				"PDU_SES_REL" => Ok(Self::PduSesRel),
				"PDU_SES_EST" => Ok(Self::PduSesEst),
				"UE_MEMORY_AVAILABLE_FOR_SMS" => Ok(Self::UeMemoryAvailableForSms),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ExpectedUeBehaviour
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "batteryIndication": {
	///      "$ref": "#/components/schemas/BatteryIndicationRm"
	///    },
	///    "communicationDurationTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "expectedUmts": {
	///      "description": "Identifies the UE's expected geographical movement.
	/// The attribute is only applicable in 5G.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/LocationArea"
	///      },
	///      "minItems": 1
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "periodicTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "scheduledCommunicationTime": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTimeRm"
	///    },
	///    "scheduledCommunicationType": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTypeRm"
	///    },
	///    "stationaryIndication": {
	///      "$ref": "#/components/schemas/StationaryIndicationRm"
	///    },
	///    "trafficProfile": {
	///      "$ref": "#/components/schemas/TrafficProfileRm"
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
	pub struct ExpectedUeBehaviour(pub Option<ExpectedUeBehaviourInner>);

	impl ::std::ops::Deref for ExpectedUeBehaviour {
		type Target = Option<ExpectedUeBehaviourInner>;
		fn deref(&self) -> &Option<ExpectedUeBehaviourInner> {
			&self.0
		}
	}

	impl From<ExpectedUeBehaviour> for Option<ExpectedUeBehaviourInner> {
		fn from(value: ExpectedUeBehaviour) -> Self {
			value.0
		}
	}

	impl From<&ExpectedUeBehaviour> for ExpectedUeBehaviour {
		fn from(value: &ExpectedUeBehaviour) -> Self {
			value.clone()
		}
	}

	impl From<Option<ExpectedUeBehaviourInner>> for ExpectedUeBehaviour {
		fn from(value: Option<ExpectedUeBehaviourInner>) -> Self {
			Self(value)
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

	/// ExpectedUeBehaviourInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "batteryIndication": {
	///      "$ref": "#/components/schemas/BatteryIndicationRm"
	///    },
	///    "communicationDurationTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "expectedUmts": {
	///      "description": "Identifies the UE's expected geographical movement.
	/// The attribute is only applicable in 5G.",
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/LocationArea"
	///      },
	///      "minItems": 1
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "periodicTime": {
	///      "$ref": "#/components/schemas/DurationSecRm"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "scheduledCommunicationTime": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTimeRm"
	///    },
	///    "scheduledCommunicationType": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTypeRm"
	///    },
	///    "stationaryIndication": {
	///      "$ref": "#/components/schemas/StationaryIndicationRm"
	///    },
	///    "trafficProfile": {
	///      "$ref": "#/components/schemas/TrafficProfileRm"
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
	pub struct ExpectedUeBehaviourInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(
			rename = "batteryIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub battery_indication: Option<BatteryIndicationRm>,
		#[serde(
			rename = "communicationDurationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub communication_duration_time: Option<DurationSecRm>,
		/// Identifies the UE's expected geographical movement. The attribute is
		/// only applicable in 5G.
		#[serde(
			rename = "expectedUmts",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expected_umts: Option<Vec<LocationArea>>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(
			rename = "periodicTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub periodic_time: Option<DurationSecRm>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(
			rename = "scheduledCommunicationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_communication_time: Option<ScheduledCommunicationTimeRm>,
		#[serde(
			rename = "scheduledCommunicationType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_communication_type: Option<ScheduledCommunicationTypeRm>,
		#[serde(
			rename = "stationaryIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub stationary_indication: Option<StationaryIndicationRm>,
		#[serde(
			rename = "trafficProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_profile: Option<TrafficProfileRm>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&ExpectedUeBehaviourInner> for ExpectedUeBehaviourInner {
		fn from(value: &ExpectedUeBehaviourInner) -> Self {
			value.clone()
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
		NewUnchecked
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

	/// Contains identifiers of shared Session Management Subscription Data and
	/// optionally individual Session Management Subscription Data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains identifiers of shared Session Management
	/// Subscription Data and optionally individual Session Management
	/// Subscription Data.",
	///  "type": "object",
	///  "required": [
	///    "sharedSmSubsDataIds"
	///  ],
	///  "properties": {
	///    "individualSmSubsData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SessionManagementSubscriptionData"
	///      }
	///    },
	///    "sharedSmSubsDataIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataId"
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
	pub struct ExtendedSmSubsData {
		#[serde(
			rename = "individualSmSubsData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub individual_sm_subs_data: Vec<SessionManagementSubscriptionData>,
		#[serde(rename = "sharedSmSubsDataIds")]
		pub shared_sm_subs_data_ids: Vec<SharedDataId>,
	}

	impl From<&ExtendedSmSubsData> for ExtendedSmSubsData {
		fn from(value: &ExtendedSmSubsData) -> Self {
			value.clone()
		}
	}

	/// ExternalUnrelatedClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "properties": {
	///    "afExternals": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfExternal"
	///      },
	///      "minItems": 1
	///    },
	///    "lcsClientExternals": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LcsClientExternal"
	///      },
	///      "minItems": 1
	///    },
	///    "lcsClientGroupExternals": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LcsClientGroupExternal"
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
	pub struct ExternalUnrelatedClass {
		#[serde(rename = "afExternals", default, skip_serializing_if = "Vec::is_empty")]
		pub af_externals: Vec<AfExternal>,
		#[serde(
			rename = "lcsClientExternals",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub lcs_client_externals: Vec<LcsClientExternal>,
		#[serde(
			rename = "lcsClientGroupExternals",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub lcs_client_group_externals: Vec<LcsClientGroupExternal>,
	}

	impl From<&ExternalUnrelatedClass> for ExternalUnrelatedClass {
		fn from(value: &ExternalUnrelatedClass) -> Self {
			value.clone()
		}
	}

	/// Indicates the Failed cause of the failed Monitoring Configuration in the
	/// EE subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the Failed cause of the failed Monitoring
	/// Configuration in the EE subscription",
	///  "type": "string",
	///  "enum": [
	///    "AF_NOT_ALLOWED",
	///    "MTC_PROVIDER_NOT_ALLOWED",
	///    "MONITORING_NOT_ALLOWED",
	///    "UNSUPPORTED_MONITORING_EVENT_TYPE",
	///    "UNSUPPORTED_MONITORING_REPORT_OPTIONS",
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
	pub enum FailedCause {
		#[default]
		#[serde(rename = "AF_NOT_ALLOWED")]
		AfNotAllowed,
		#[serde(rename = "MTC_PROVIDER_NOT_ALLOWED")]
		MtcProviderNotAllowed,
		#[serde(rename = "MONITORING_NOT_ALLOWED")]
		MonitoringNotAllowed,
		#[serde(rename = "UNSUPPORTED_MONITORING_EVENT_TYPE")]
		UnsupportedMonitoringEventType,
		#[serde(rename = "UNSUPPORTED_MONITORING_REPORT_OPTIONS")]
		UnsupportedMonitoringReportOptions,
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FailedCause> for FailedCause {
		fn from(value: &FailedCause) -> Self {
			value.clone()
		}
	}

	impl ToString for FailedCause {
		fn to_string(&self) -> String {
			match *self {
				Self::AfNotAllowed => "AF_NOT_ALLOWED".to_string(),
				Self::MtcProviderNotAllowed => "MTC_PROVIDER_NOT_ALLOWED".to_string(),
				Self::MonitoringNotAllowed => "MONITORING_NOT_ALLOWED".to_string(),
				Self::UnsupportedMonitoringEventType => {
					"UNSUPPORTED_MONITORING_EVENT_TYPE".to_string()
				}
				Self::UnsupportedMonitoringReportOptions => {
					"UNSUPPORTED_MONITORING_REPORT_OPTIONS".to_string()
				}
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FailedCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AF_NOT_ALLOWED" => Ok(Self::AfNotAllowed),
				"MTC_PROVIDER_NOT_ALLOWED" => Ok(Self::MtcProviderNotAllowed),
				"MONITORING_NOT_ALLOWED" => Ok(Self::MonitoringNotAllowed),
				"UNSUPPORTED_MONITORING_EVENT_TYPE" => Ok(Self::UnsupportedMonitoringEventType),
				"UNSUPPORTED_MONITORING_REPORT_OPTIONS" => {
					Ok(Self::UnsupportedMonitoringReportOptions)
				}
				"UNSPECIFIED" => Ok(Self::Unspecified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FailedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FailedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FailedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the event type and failed cause of the failed Monitoring
	/// Configuration in the EE subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the event type and failed cause of the failed
	/// Monitoring Configuration in the EE subscription",
	///  "type": "object",
	///  "required": [
	///    "eventType",
	///    "failedCause"
	///  ],
	///  "properties": {
	///    "eventType": {
	///      "$ref": "#/components/schemas/EventType"
	///    },
	///    "failedCause": {
	///      "$ref": "#/components/schemas/FailedCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct FailedMonitoringConfiguration {
		#[serde(rename = "eventType")]
		pub event_type: EventType,
		#[serde(rename = "failedCause")]
		pub failed_cause: FailedCause,
	}

	impl From<&FailedMonitoringConfiguration> for FailedMonitoringConfiguration {
		fn from(value: &FailedMonitoringConfiguration) -> Self {
			value.clone()
		}
	}

	/// FrameRouteInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "ipv4Mask": {
	///      "$ref": "#/components/schemas/Ipv4AddrMask"
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
	pub struct FrameRouteInfo {
		#[serde(rename = "ipv4Mask", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_mask: Option<Ipv4AddrMask>,
		#[serde(
			rename = "ipv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_prefix: Option<Ipv6Prefix>,
	}

	impl From<&FrameRouteInfo> for FrameRouteInfo {
		fn from(value: &FrameRouteInfo) -> Self {
			value.clone()
		}
	}

	/// GbaAuthType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DIGEST_AKAV1_MD5"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum GbaAuthType {
		#[default]
		#[serde(rename = "DIGEST_AKAV1_MD5")]
		DigestAkav1Md5,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&GbaAuthType> for GbaAuthType {
		fn from(value: &GbaAuthType) -> Self {
			value.clone()
		}
	}

	impl ToString for GbaAuthType {
		fn to_string(&self) -> String {
			match *self {
				Self::DigestAkav1Md5 => "DIGEST_AKAV1_MD5".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for GbaAuthType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DIGEST_AKAV1_MD5" => Ok(Self::DigestAkav1Md5),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for GbaAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for GbaAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for GbaAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// GbaAuthenticationInfoRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "authType"
	///  ],
	///  "properties": {
	///    "authType": {
	///      "$ref": "#/components/schemas/GbaAuthType"
	///    },
	///    "resynchronizationInfo": {
	///      "$ref": "#/components/schemas/schemas-ResynchronizationInfo"
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
	pub struct GbaAuthenticationInfoRequest {
		#[serde(rename = "authType")]
		pub auth_type: GbaAuthType,
		#[serde(
			rename = "resynchronizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resynchronization_info: Option<SchemasResynchronizationInfo>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&GbaAuthenticationInfoRequest> for GbaAuthenticationInfoRequest {
		fn from(value: &GbaAuthenticationInfoRequest) -> Self {
			value.clone()
		}
	}

	/// GbaAuthenticationInfoResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "3gAkaAv": {
	///      "$ref": "#/components/schemas/3GAkaAv"
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
	pub struct GbaAuthenticationInfoResult {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "3gAkaAv", default, skip_serializing_if = "Option::is_none")]
		pub three_g_aka_av: Option<_3gAkaAv>,
	}

	impl From<&GbaAuthenticationInfoResult> for GbaAuthenticationInfoResult {
		fn from(value: &GbaAuthenticationInfoResult) -> Self {
			value.clone()
		}
	}

	/// Type of GPSI (MSISDN or External-ID)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type of GPSI (MSISDN or External-ID)",
	///  "type": "string",
	///  "enum": [
	///    "MSISDN",
	///    "EXT_ID",
	///    "EXT_GROUP_ID"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum GpsiType {
		#[default]
		#[serde(rename = "MSISDN")]
		Msisdn,
		#[serde(rename = "EXT_ID")]
		ExtId,
		#[serde(rename = "EXT_GROUP_ID")]
		ExtGroupId,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&GpsiType> for GpsiType {
		fn from(value: &GpsiType) -> Self {
			value.clone()
		}
	}

	impl ToString for GpsiType {
		fn to_string(&self) -> String {
			match *self {
				Self::Msisdn => "MSISDN".to_string(),
				Self::ExtId => "EXT_ID".to_string(),
				Self::ExtGroupId => "EXT_GROUP_ID".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for GpsiType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MSISDN" => Ok(Self::Msisdn),
				"EXT_ID" => Ok(Self::ExtId),
				"EXT_GROUP_ID" => Ok(Self::ExtGroupId),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for GpsiType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for GpsiType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for GpsiType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// GroupIdentifiers
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "extGroupId": {
	///      "$ref": "#/components/schemas/ExtGroupId"
	///    },
	///    "intGroupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "ueIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeId"
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
	pub struct GroupIdentifiers {
		#[serde(
			rename = "extGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_group_id: Option<ExtGroupId>,
		#[serde(
			rename = "intGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub int_group_id: Option<GroupId>,
		#[serde(rename = "ueIdList", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_id_list: Vec<UeId>,
	}

	impl From<&GroupIdentifiers> for GroupIdentifiers {
		fn from(value: &GroupIdentifiers) -> Self {
			value.clone()
		}
	}

	/// HssAuthType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "EPS_AKA",
	///    "EAP_AKA",
	///    "EAP_AKA_PRIME",
	///    "IMS_AKA",
	///    "GBA_AKA",
	///    "UMTS_AKA"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum HssAuthType {
		#[default]
		#[serde(rename = "EPS_AKA")]
		EpsAka,
		#[serde(rename = "EAP_AKA")]
		EapAka,
		#[serde(rename = "EAP_AKA_PRIME")]
		EapAkaPrime,
		#[serde(rename = "IMS_AKA")]
		ImsAka,
		#[serde(rename = "GBA_AKA")]
		GbaAka,
		#[serde(rename = "UMTS_AKA")]
		UmtsAka,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&HssAuthType> for HssAuthType {
		fn from(value: &HssAuthType) -> Self {
			value.clone()
		}
	}

	impl ToString for HssAuthType {
		fn to_string(&self) -> String {
			match *self {
				Self::EpsAka => "EPS_AKA".to_string(),
				Self::EapAka => "EAP_AKA".to_string(),
				Self::EapAkaPrime => "EAP_AKA_PRIME".to_string(),
				Self::ImsAka => "IMS_AKA".to_string(),
				Self::GbaAka => "GBA_AKA".to_string(),
				Self::UmtsAka => "UMTS_AKA".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for HssAuthType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EPS_AKA" => Ok(Self::EpsAka),
				"EAP_AKA" => Ok(Self::EapAka),
				"EAP_AKA_PRIME" => Ok(Self::EapAkaPrime),
				"IMS_AKA" => Ok(Self::ImsAka),
				"GBA_AKA" => Ok(Self::GbaAka),
				"UMTS_AKA" => Ok(Self::UmtsAka),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for HssAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HssAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HssAuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// HssAuthTypeInUri
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "eps-aka",
	///    "eap-aka",
	///    "eap-aka-prime",
	///    "ims-aka",
	///    "gba-aka"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum HssAuthTypeInUri {
		#[default]
		#[serde(rename = "eps-aka")]
		EpsAka,
		#[serde(rename = "eap-aka")]
		EapAka,
		#[serde(rename = "eap-aka-prime")]
		EapAkaPrime,
		#[serde(rename = "ims-aka")]
		ImsAka,
		#[serde(rename = "gba-aka")]
		GbaAka,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&HssAuthTypeInUri> for HssAuthTypeInUri {
		fn from(value: &HssAuthTypeInUri) -> Self {
			value.clone()
		}
	}

	impl ToString for HssAuthTypeInUri {
		fn to_string(&self) -> String {
			match *self {
				Self::EpsAka => "eps-aka".to_string(),
				Self::EapAka => "eap-aka".to_string(),
				Self::EapAkaPrime => "eap-aka-prime".to_string(),
				Self::ImsAka => "ims-aka".to_string(),
				Self::GbaAka => "gba-aka".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for HssAuthTypeInUri {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"eps-aka" => Ok(Self::EpsAka),
				"eap-aka" => Ok(Self::EapAka),
				"eap-aka-prime" => Ok(Self::EapAkaPrime),
				"ims-aka" => Ok(Self::ImsAka),
				"gba-aka" => Ok(Self::GbaAka),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for HssAuthTypeInUri {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HssAuthTypeInUri {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HssAuthTypeInUri {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// HssAuthenticationInfoRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "hssAuthType",
	///    "numOfRequestedVectors"
	///  ],
	///  "properties": {
	///    "anId": {
	///      "$ref": "#/components/schemas/AccessNetworkId"
	///    },
	///    "hssAuthType": {
	///      "$ref": "#/components/schemas/HssAuthType"
	///    },
	///    "numOfRequestedVectors": {
	///      "$ref": "#/components/schemas/NumOfRequestedVectors"
	///    },
	///    "requestingNodeType": {
	///      "$ref": "#/components/schemas/NodeType"
	///    },
	///    "resynchronizationInfo": {
	///      "$ref": "#/components/schemas/ResynchronizationInfo"
	///    },
	///    "servingNetworkId": {
	///      "$ref": "#/components/schemas/PlmnId"
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
	pub struct HssAuthenticationInfoRequest {
		#[serde(rename = "anId", default, skip_serializing_if = "Option::is_none")]
		pub an_id: Option<AccessNetworkId>,
		#[serde(rename = "hssAuthType")]
		pub hss_auth_type: HssAuthType,
		#[serde(rename = "numOfRequestedVectors")]
		pub num_of_requested_vectors: NumOfRequestedVectors,
		#[serde(
			rename = "requestingNodeType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requesting_node_type: Option<NodeType>,
		#[serde(
			rename = "resynchronizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resynchronization_info: Option<ResynchronizationInfo>,
		#[serde(
			rename = "servingNetworkId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network_id: Option<PlmnId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&HssAuthenticationInfoRequest> for HssAuthenticationInfoRequest {
		fn from(value: &HssAuthenticationInfoRequest) -> Self {
			value.clone()
		}
	}

	/// HssAuthenticationInfoResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "hssAuthenticationVectors"
	///  ],
	///  "properties": {
	///    "hssAuthenticationVectors": {
	///      "$ref": "#/components/schemas/HssAuthenticationVectors"
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
	pub struct HssAuthenticationInfoResult {
		#[serde(rename = "hssAuthenticationVectors")]
		pub hss_authentication_vectors: HssAuthenticationVectors,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&HssAuthenticationInfoResult> for HssAuthenticationInfoResult {
		fn from(value: &HssAuthenticationInfoResult) -> Self {
			value.clone()
		}
	}

	/// HssAuthenticationVectors
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AvEpsAka"
	///      },
	///      "maxItems": 5,
	///      "minItems": 1
	///    },
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AvImsGbaEapAka"
	///      },
	///      "maxItems": 5,
	///      "minItems": 1
	///    },
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AvEapAkaPrime"
	///      },
	///      "maxItems": 5,
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
	pub enum HssAuthenticationVectors {
		#[default]
		Variant0(Vec<AvEpsAka>),
		Variant1(Vec<AvImsGbaEapAka>),
		Variant2(Vec<AvEapAkaPrime>),
	}

	impl From<&HssAuthenticationVectors> for HssAuthenticationVectors {
		fn from(value: &HssAuthenticationVectors) -> Self {
			value.clone()
		}
	}

	impl From<Vec<AvEpsAka>> for HssAuthenticationVectors {
		fn from(value: Vec<AvEpsAka>) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<Vec<AvImsGbaEapAka>> for HssAuthenticationVectors {
		fn from(value: Vec<AvImsGbaEapAka>) -> Self {
			Self::Variant1(value)
		}
	}

	impl From<Vec<AvEapAkaPrime>> for HssAuthenticationVectors {
		fn from(value: Vec<AvEapAkaPrime>) -> Self {
			Self::Variant2(value)
		}
	}

	/// HssAvType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "EPS_AKA",
	///    "EAP_AKA",
	///    "IMS_AKA",
	///    "GBA_AKA",
	///    "UMTS_AKA"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum HssAvType {
		#[default]
		#[serde(rename = "EPS_AKA")]
		EpsAka,
		#[serde(rename = "EAP_AKA")]
		EapAka,
		#[serde(rename = "IMS_AKA")]
		ImsAka,
		#[serde(rename = "GBA_AKA")]
		GbaAka,
		#[serde(rename = "UMTS_AKA")]
		UmtsAka,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&HssAvType> for HssAvType {
		fn from(value: &HssAvType) -> Self {
			value.clone()
		}
	}

	impl ToString for HssAvType {
		fn to_string(&self) -> String {
			match *self {
				Self::EpsAka => "EPS_AKA".to_string(),
				Self::EapAka => "EAP_AKA".to_string(),
				Self::ImsAka => "IMS_AKA".to_string(),
				Self::GbaAka => "GBA_AKA".to_string(),
				Self::UmtsAka => "UMTS_AKA".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for HssAvType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EPS_AKA" => Ok(Self::EpsAka),
				"EAP_AKA" => Ok(Self::EapAka),
				"IMS_AKA" => Ok(Self::ImsAka),
				"GBA_AKA" => Ok(Self::GbaAka),
				"UMTS_AKA" => Ok(Self::UmtsAka),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for HssAvType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HssAvType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HssAvType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// IdTranslationResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "additionalGpsis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalSupis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
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
	pub struct IdTranslationResult {
		#[serde(
			rename = "additionalGpsis",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_gpsis: Vec<Gpsi>,
		#[serde(
			rename = "additionalSupis",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_supis: Vec<Supi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		pub supi: Supi,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&IdTranslationResult> for IdTranslationResult {
		fn from(value: &IdTranslationResult) -> Self {
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
		NewUnchecked
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
		NewUnchecked
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

	/// IkPrime
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
		NewUnchecked
	)]
	pub struct IkPrime(String);

	impl ::std::ops::Deref for IkPrime {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IkPrime> for String {
		fn from(value: IkPrime) -> Self {
			value.0
		}
	}

	impl From<&IkPrime> for IkPrime {
		fn from(value: &IkPrime) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for IkPrime {
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

	impl ::std::convert::TryFrom<&str> for IkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for IkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for IkPrime {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for IkPrime {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// ImmediateReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/SubscriptionDataSets"
	///    },
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedData"
	///      },
	///      "minItems": 0
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum ImmediateReport {
		#[default]
		Variant0(SubscriptionDataSets),
		Variant1(Vec<SharedData>),
	}

	impl From<&ImmediateReport> for ImmediateReport {
		fn from(value: &ImmediateReport) -> Self {
			value.clone()
		}
	}

	impl From<SubscriptionDataSets> for ImmediateReport {
		fn from(value: SubscriptionDataSets) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<Vec<SharedData>> for ImmediateReport {
		fn from(value: Vec<SharedData>) -> Self {
			Self::Variant1(value)
		}
	}

	/// ImsVoPs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "HOMOGENEOUS_SUPPORT",
	///    "HOMOGENEOUS_NON_SUPPORT",
	///    "NON_HOMOGENEOUS_OR_UNKNOWN"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ImsVoPs {
		#[default]
		#[serde(rename = "HOMOGENEOUS_SUPPORT")]
		HomogeneousSupport,
		#[serde(rename = "HOMOGENEOUS_NON_SUPPORT")]
		HomogeneousNonSupport,
		#[serde(rename = "NON_HOMOGENEOUS_OR_UNKNOWN")]
		NonHomogeneousOrUnknown,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ImsVoPs> for ImsVoPs {
		fn from(value: &ImsVoPs) -> Self {
			value.clone()
		}
	}

	impl ToString for ImsVoPs {
		fn to_string(&self) -> String {
			match *self {
				Self::HomogeneousSupport => "HOMOGENEOUS_SUPPORT".to_string(),
				Self::HomogeneousNonSupport => "HOMOGENEOUS_NON_SUPPORT".to_string(),
				Self::NonHomogeneousOrUnknown => "NON_HOMOGENEOUS_OR_UNKNOWN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ImsVoPs {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"HOMOGENEOUS_SUPPORT" => Ok(Self::HomogeneousSupport),
				"HOMOGENEOUS_NON_SUPPORT" => Ok(Self::HomogeneousNonSupport),
				"NON_HOMOGENEOUS_OR_UNKNOWN" => Ok(Self::NonHomogeneousOrUnknown),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ImsVoPs {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ImsVoPs {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ImsVoPs {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// IntegrityKey
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
		NewUnchecked
	)]
	pub struct IntegrityKey(String);

	impl ::std::ops::Deref for IntegrityKey {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IntegrityKey> for String {
		fn from(value: IntegrityKey) -> Self {
			value.0
		}
	}

	impl From<&IntegrityKey> for IntegrityKey {
		fn from(value: &IntegrityKey) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for IntegrityKey {
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

	impl ::std::convert::TryFrom<&str> for IntegrityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for IntegrityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for IntegrityKey {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for IntegrityKey {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Possible values are - SUBSRIPTION_WITHDRAWAL - DNN_REMOVED -
	/// SLICE_REMOVED - AUTHORIZATION_REVOKED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are - SUBSRIPTION_WITHDRAWAL -
	/// DNN_REMOVED - SLICE_REMOVED - AUTHORIZATION_REVOKED\n",
	///  "type": "string",
	///  "enum": [
	///    "SUBSRIPTION_WITHDRAWAL",
	///    "DNN_REMOVED",
	///    "SLICE_REMOVED",
	///    "AUTHORIZATION_REVOKED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum InvalidCause {
		#[default]
		#[serde(rename = "SUBSRIPTION_WITHDRAWAL")]
		SubsriptionWithdrawal,
		#[serde(rename = "DNN_REMOVED")]
		DnnRemoved,
		#[serde(rename = "SLICE_REMOVED")]
		SliceRemoved,
		#[serde(rename = "AUTHORIZATION_REVOKED")]
		AuthorizationRevoked,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&InvalidCause> for InvalidCause {
		fn from(value: &InvalidCause) -> Self {
			value.clone()
		}
	}

	impl ToString for InvalidCause {
		fn to_string(&self) -> String {
			match *self {
				Self::SubsriptionWithdrawal => "SUBSRIPTION_WITHDRAWAL".to_string(),
				Self::DnnRemoved => "DNN_REMOVED".to_string(),
				Self::SliceRemoved => "SLICE_REMOVED".to_string(),
				Self::AuthorizationRevoked => "AUTHORIZATION_REVOKED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for InvalidCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUBSRIPTION_WITHDRAWAL" => Ok(Self::SubsriptionWithdrawal),
				"DNN_REMOVED" => Ok(Self::DnnRemoved),
				"SLICE_REMOVED" => Ok(Self::SliceRemoved),
				"AUTHORIZATION_REVOKED" => Ok(Self::AuthorizationRevoked),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for InvalidCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for InvalidCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for InvalidCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains guidance information (e.g. minimum and recommended delivery
	/// times) of the IP-SM-GW
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains guidance information (e.g. minimum and
	/// recommended delivery times) of the IP-SM-GW\n",
	///  "type": "object",
	///  "required": [
	///    "minDeliveryTime",
	///    "recommDeliveryTime"
	///  ],
	///  "properties": {
	///    "minDeliveryTime": {
	///      "type": "integer",
	///      "maximum": 600.0,
	///      "minimum": 30.0
	///    },
	///    "recommDeliveryTime": {
	///      "type": "integer",
	///      "maximum": 600.0,
	///      "minimum": 30.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IpSmGwGuidance {
		#[serde(rename = "minDeliveryTime")]
		pub min_delivery_time: i64,
		#[serde(rename = "recommDeliveryTime")]
		pub recomm_delivery_time: i64,
	}

	impl From<&IpSmGwGuidance> for IpSmGwGuidance {
		fn from(value: &IpSmGwGuidance) -> Self {
			value.clone()
		}
	}

	/// Contains the IP-SM-GW Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the IP-SM-GW Information",
	///  "type": "object",
	///  "properties": {
	///    "ipSmGwGuidance": {
	///      "$ref": "#/components/schemas/IpSmGwGuidance"
	///    },
	///    "ipSmGwRegistration": {
	///      "$ref": "#/components/schemas/IpSmGwRegistration"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IpSmGwInfo {
		#[serde(
			rename = "ipSmGwGuidance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ip_sm_gw_guidance: Option<IpSmGwGuidance>,
		#[serde(
			rename = "ipSmGwRegistration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ip_sm_gw_registration: Option<IpSmGwRegistration>,
	}

	impl From<&IpSmGwInfo> for IpSmGwInfo {
		fn from(value: &IpSmGwInfo) -> Self {
			value.clone()
		}
	}

	/// IpSmGwRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ipSmGwMapAddress"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipSmGwDiameterAddress"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipsmgwIpv4"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipsmgwIpv6"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipsmgwFqdn"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ipSmGwDiameterAddress": {
	///      "$ref": "#/components/schemas/NetworkNodeDiameterAddress"
	///    },
	///    "ipSmGwMapAddress": {
	///      "$ref": "#/components/schemas/E164Number"
	///    },
	///    "ipSmGwSbiSupInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ipsmgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipsmgwIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipsmgwIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "unriIndicator": {
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
	#[serde(untagged)]
	pub enum IpSmGwRegistration {
		#[default]
		Variant0 {
			#[serde(rename = "ipSmGwMapAddress")]
			ip_sm_gw_map_address: E164Number,
			#[serde(rename = "ipSmGwSbiSupInd", default)]
			ip_sm_gw_sbi_sup_ind: bool,
			#[serde(
				rename = "nfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_instance_id: Option<NfInstanceId>,
			#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
			reset_ids: Vec<String>,
			#[serde(rename = "unriIndicator", default)]
			unri_indicator: bool,
		},
		Variant1 {
			#[serde(rename = "ipSmGwDiameterAddress")]
			ip_sm_gw_diameter_address: NetworkNodeDiameterAddress,
			#[serde(rename = "ipSmGwSbiSupInd", default)]
			ip_sm_gw_sbi_sup_ind: bool,
			#[serde(
				rename = "nfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_instance_id: Option<NfInstanceId>,
			#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
			reset_ids: Vec<String>,
			#[serde(rename = "unriIndicator", default)]
			unri_indicator: bool,
		},
		Variant2 {
			#[serde(rename = "ipSmGwSbiSupInd", default)]
			ip_sm_gw_sbi_sup_ind: bool,
			#[serde(rename = "ipsmgwIpv4")]
			ipsmgw_ipv4: Ipv4Addr,
			#[serde(
				rename = "nfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_instance_id: Option<NfInstanceId>,
			#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
			reset_ids: Vec<String>,
			#[serde(rename = "unriIndicator", default)]
			unri_indicator: bool,
		},
		Variant3 {
			#[serde(rename = "ipSmGwSbiSupInd", default)]
			ip_sm_gw_sbi_sup_ind: bool,
			#[serde(rename = "ipsmgwIpv6")]
			ipsmgw_ipv6: Ipv6Addr,
			#[serde(
				rename = "nfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_instance_id: Option<NfInstanceId>,
			#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
			reset_ids: Vec<String>,
			#[serde(rename = "unriIndicator", default)]
			unri_indicator: bool,
		},
		Variant4 {
			#[serde(rename = "ipSmGwSbiSupInd", default)]
			ip_sm_gw_sbi_sup_ind: bool,
			#[serde(rename = "ipsmgwFqdn")]
			ipsmgw_fqdn: Fqdn,
			#[serde(
				rename = "nfInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_instance_id: Option<NfInstanceId>,
			#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
			reset_ids: Vec<String>,
			#[serde(rename = "unriIndicator", default)]
			unri_indicator: bool,
		},
	}

	impl From<&IpSmGwRegistration> for IpSmGwRegistration {
		fn from(value: &IpSmGwRegistration) -> Self {
			value.clone()
		}
	}

	/// IwkEpsInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IwkEpsInd(pub bool);

	impl ::std::ops::Deref for IwkEpsInd {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<IwkEpsInd> for bool {
		fn from(value: IwkEpsInd) -> Self {
			value.0
		}
	}

	impl From<&IwkEpsInd> for IwkEpsInd {
		fn from(value: &IwkEpsInd) -> Self {
			value.clone()
		}
	}

	impl From<bool> for IwkEpsInd {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for IwkEpsInd {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for IwkEpsInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for IwkEpsInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for IwkEpsInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for IwkEpsInd {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Kasme
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{64}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct Kasme(String);

	impl ::std::ops::Deref for Kasme {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Kasme> for String {
		fn from(value: Kasme) -> Self {
			value.0
		}
	}

	impl From<&Kasme> for Kasme {
		fn from(value: &Kasme) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Kasme {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{64}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{64}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Kasme {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Kasme {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Kasme {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Kasme {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Kausf
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{64}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct Kausf(String);

	impl ::std::ops::Deref for Kausf {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Kausf> for String {
		fn from(value: Kausf) -> Self {
			value.0
		}
	}

	impl From<&Kausf> for Kausf {
		fn from(value: &Kausf) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Kausf {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{64}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{64}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Kausf {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Kausf {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Kausf {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Kausf {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// LboRoamingAllowed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LboRoamingAllowed(pub bool);

	impl ::std::ops::Deref for LboRoamingAllowed {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<LboRoamingAllowed> for bool {
		fn from(value: LboRoamingAllowed) -> Self {
			value.0
		}
	}

	impl From<&LboRoamingAllowed> for LboRoamingAllowed {
		fn from(value: &LboRoamingAllowed) -> Self {
			value.clone()
		}
	}

	impl From<bool> for LboRoamingAllowed {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LboRoamingAllowed {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for LboRoamingAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LboRoamingAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LboRoamingAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for LboRoamingAllowed {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// LcsBroadcastAssistanceTypesData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "locationAssistanceType"
	///  ],
	///  "properties": {
	///    "locationAssistanceType": {
	///      "$ref": "#/components/schemas/Binary"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsBroadcastAssistanceTypesData {
		#[serde(rename = "locationAssistanceType")]
		pub location_assistance_type: Binary,
	}

	impl From<&LcsBroadcastAssistanceTypesData> for LcsBroadcastAssistanceTypesData {
		fn from(value: &LcsBroadcastAssistanceTypesData) -> Self {
			value.clone()
		}
	}

	/// LcsClientClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "BROADCAST_SERVICE",
	///    "OM_IN_HPLMN",
	///    "OM_IN_VPLMN",
	///    "ANONYMOUS_LOCATION_SERVICE",
	///    "SPECIFIC_SERVICE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum LcsClientClass {
		#[default]
		#[serde(rename = "BROADCAST_SERVICE")]
		BroadcastService,
		#[serde(rename = "OM_IN_HPLMN")]
		OmInHplmn,
		#[serde(rename = "OM_IN_VPLMN")]
		OmInVplmn,
		#[serde(rename = "ANONYMOUS_LOCATION_SERVICE")]
		AnonymousLocationService,
		#[serde(rename = "SPECIFIC_SERVICE")]
		SpecificService,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LcsClientClass> for LcsClientClass {
		fn from(value: &LcsClientClass) -> Self {
			value.clone()
		}
	}

	impl ToString for LcsClientClass {
		fn to_string(&self) -> String {
			match *self {
				Self::BroadcastService => "BROADCAST_SERVICE".to_string(),
				Self::OmInHplmn => "OM_IN_HPLMN".to_string(),
				Self::OmInVplmn => "OM_IN_VPLMN".to_string(),
				Self::AnonymousLocationService => "ANONYMOUS_LOCATION_SERVICE".to_string(),
				Self::SpecificService => "SPECIFIC_SERVICE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LcsClientClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BROADCAST_SERVICE" => Ok(Self::BroadcastService),
				"OM_IN_HPLMN" => Ok(Self::OmInHplmn),
				"OM_IN_VPLMN" => Ok(Self::OmInVplmn),
				"ANONYMOUS_LOCATION_SERVICE" => Ok(Self::AnonymousLocationService),
				"SPECIFIC_SERVICE" => Ok(Self::SpecificService),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LcsClientClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsClientClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsClientClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LcsClientExternal
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "allowedGeographicArea": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 1
	///    },
	///    "privacyCheckRelatedAction": {
	///      "$ref": "#/components/schemas/PrivacyCheckRelatedAction"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsClientExternal {
		#[serde(
			rename = "allowedGeographicArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_geographic_area: Vec<GeographicArea>,
		#[serde(
			rename = "privacyCheckRelatedAction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub privacy_check_related_action: Option<PrivacyCheckRelatedAction>,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&LcsClientExternal> for LcsClientExternal {
		fn from(value: &LcsClientExternal) -> Self {
			value.clone()
		}
	}

	/// LcsClientGroupExternal
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "allowedGeographicArea": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 1
	///    },
	///    "lcsClientGroupId": {
	///      "$ref": "#/components/schemas/ExtGroupId"
	///    },
	///    "privacyCheckRelatedAction": {
	///      "$ref": "#/components/schemas/PrivacyCheckRelatedAction"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsClientGroupExternal {
		#[serde(
			rename = "allowedGeographicArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_geographic_area: Vec<GeographicArea>,
		#[serde(
			rename = "lcsClientGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_client_group_id: Option<ExtGroupId>,
		#[serde(
			rename = "privacyCheckRelatedAction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub privacy_check_related_action: Option<PrivacyCheckRelatedAction>,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&LcsClientGroupExternal> for LcsClientGroupExternal {
		fn from(value: &LcsClientGroupExternal) -> Self {
			value.clone()
		}
	}

	/// LcsClientId
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
	pub struct LcsClientId(pub String);

	impl ::std::ops::Deref for LcsClientId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<LcsClientId> for String {
		fn from(value: LcsClientId) -> Self {
			value.0
		}
	}

	impl From<&LcsClientId> for LcsClientId {
		fn from(value: &LcsClientId) -> Self {
			value.clone()
		}
	}

	impl From<String> for LcsClientId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LcsClientId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for LcsClientId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// LcsMoData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "allowedServiceClasses"
	///  ],
	///  "properties": {
	///    "allowedServiceClasses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LcsMoServiceClass"
	///      },
	///      "minItems": 1
	///    },
	///    "moAssistanceDataTypes": {
	///      "$ref": "#/components/schemas/LcsBroadcastAssistanceTypesData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsMoData {
		#[serde(rename = "allowedServiceClasses")]
		pub allowed_service_classes: Vec<LcsMoServiceClass>,
		#[serde(
			rename = "moAssistanceDataTypes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_assistance_data_types: Option<LcsBroadcastAssistanceTypesData>,
	}

	impl From<&LcsMoData> for LcsMoData {
		fn from(value: &LcsMoData) -> Self {
			value.clone()
		}
	}

	/// LcsMoServiceClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "BASIC_SELF_LOCATION",
	///    "AUTONOMOUS_SELF_LOCATION",
	///    "TRANSFER_TO_THIRD_PARTY"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum LcsMoServiceClass {
		#[default]
		#[serde(rename = "BASIC_SELF_LOCATION")]
		BasicSelfLocation,
		#[serde(rename = "AUTONOMOUS_SELF_LOCATION")]
		AutonomousSelfLocation,
		#[serde(rename = "TRANSFER_TO_THIRD_PARTY")]
		TransferToThirdParty,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LcsMoServiceClass> for LcsMoServiceClass {
		fn from(value: &LcsMoServiceClass) -> Self {
			value.clone()
		}
	}

	impl ToString for LcsMoServiceClass {
		fn to_string(&self) -> String {
			match *self {
				Self::BasicSelfLocation => "BASIC_SELF_LOCATION".to_string(),
				Self::AutonomousSelfLocation => "AUTONOMOUS_SELF_LOCATION".to_string(),
				Self::TransferToThirdParty => "TRANSFER_TO_THIRD_PARTY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LcsMoServiceClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BASIC_SELF_LOCATION" => Ok(Self::BasicSelfLocation),
				"AUTONOMOUS_SELF_LOCATION" => Ok(Self::AutonomousSelfLocation),
				"TRANSFER_TO_THIRD_PARTY" => Ok(Self::TransferToThirdParty),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LcsMoServiceClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsMoServiceClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsMoServiceClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LcsPrivacy
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "lpi": {
	///      "$ref": "#/components/schemas/schemas-Lpi"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsPrivacy(pub Option<LcsPrivacyInner>);

	impl ::std::ops::Deref for LcsPrivacy {
		type Target = Option<LcsPrivacyInner>;
		fn deref(&self) -> &Option<LcsPrivacyInner> {
			&self.0
		}
	}

	impl From<LcsPrivacy> for Option<LcsPrivacyInner> {
		fn from(value: LcsPrivacy) -> Self {
			value.0
		}
	}

	impl From<&LcsPrivacy> for LcsPrivacy {
		fn from(value: &LcsPrivacy) -> Self {
			value.clone()
		}
	}

	impl From<Option<LcsPrivacyInner>> for LcsPrivacy {
		fn from(value: Option<LcsPrivacyInner>) -> Self {
			Self(value)
		}
	}

	/// LcsPrivacyData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "lpi": {
	///      "$ref": "#/components/schemas/Lpi"
	///    },
	///    "plmnOperatorClasses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnOperatorClass"
	///      },
	///      "minItems": 1
	///    },
	///    "unrelatedClass": {
	///      "$ref": "#/components/schemas/UnrelatedClass"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsPrivacyData {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub lpi: Option<Lpi>,
		#[serde(
			rename = "plmnOperatorClasses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub plmn_operator_classes: Vec<PlmnOperatorClass>,
		#[serde(
			rename = "unrelatedClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unrelated_class: Option<UnrelatedClass>,
	}

	impl From<&LcsPrivacyData> for LcsPrivacyData {
		fn from(value: &LcsPrivacyData) -> Self {
			value.clone()
		}
	}

	/// LcsPrivacyInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "lpi": {
	///      "$ref": "#/components/schemas/schemas-Lpi"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsPrivacyInner {
		#[serde(
			rename = "afInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_instance_id: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub lpi: Option<SchemasLpi>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(
			rename = "referenceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reference_id: Option<ReferenceId>,
	}

	impl From<&LcsPrivacyInner> for LcsPrivacyInner {
		fn from(value: &LcsPrivacyInner) -> Self {
			value.clone()
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

	/// LocationAccuracy
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CELL_LEVEL",
	///    "RAN_NODE_LEVEL",
	///    "TA_LEVEL",
	///    "N3IWF_LEVEL",
	///    "UE_IP",
	///    "UE_PORT"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum LocationAccuracy {
		#[default]
		#[serde(rename = "CELL_LEVEL")]
		CellLevel,
		#[serde(rename = "RAN_NODE_LEVEL")]
		RanNodeLevel,
		#[serde(rename = "TA_LEVEL")]
		TaLevel,
		#[serde(rename = "N3IWF_LEVEL")]
		N3iwfLevel,
		#[serde(rename = "UE_IP")]
		UeIp,
		#[serde(rename = "UE_PORT")]
		UePort,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationAccuracy> for LocationAccuracy {
		fn from(value: &LocationAccuracy) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationAccuracy {
		fn to_string(&self) -> String {
			match *self {
				Self::CellLevel => "CELL_LEVEL".to_string(),
				Self::RanNodeLevel => "RAN_NODE_LEVEL".to_string(),
				Self::TaLevel => "TA_LEVEL".to_string(),
				Self::N3iwfLevel => "N3IWF_LEVEL".to_string(),
				Self::UeIp => "UE_IP".to_string(),
				Self::UePort => "UE_PORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationAccuracy {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CELL_LEVEL" => Ok(Self::CellLevel),
				"RAN_NODE_LEVEL" => Ok(Self::RanNodeLevel),
				"TA_LEVEL" => Ok(Self::TaLevel),
				"N3IWF_LEVEL" => Ok(Self::N3iwfLevel),
				"UE_IP" => Ok(Self::UeIp),
				"UE_PORT" => Ok(Self::UePort),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationAccuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationAccuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationAccuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
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
		pub nw_area_info: Option<NetworkAreaInfo>,
		#[serde(rename = "umtTime", default, skip_serializing_if = "Option::is_none")]
		pub umt_time: Option<UmtTime>,
	}

	impl From<&LocationArea> for LocationArea {
		fn from(value: &LocationArea) -> Self {
			value.clone()
		}
	}

	/// LocationInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "registrationLocationInfoList"
	///  ],
	///  "properties": {
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "registrationLocationInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RegistrationLocationInfo"
	///      },
	///      "maxItems": 2,
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
	pub struct LocationInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "registrationLocationInfoList")]
		pub registration_location_info_list: Vec<RegistrationLocationInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&LocationInfo> for LocationInfo {
		fn from(value: &LocationInfo) -> Self {
			value.clone()
		}
	}

	/// Represents information to be sent in a location information request. It
	/// contains the requested information, i.e. current location, local time
	/// zone, RAT type, or serving node identity only.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information to be sent in a location information request. It contains the requested information, i.e. current location, local time zone, RAT type, or serving node identity only.",
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
	///    "reqServingNode": {
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
	pub struct LocationInfoRequest {
		#[serde(rename = "req5gsLoc", default)]
		pub req5gs_loc: bool,
		#[serde(rename = "reqCurrentLoc", default)]
		pub req_current_loc: bool,
		#[serde(rename = "reqRatType", default)]
		pub req_rat_type: bool,
		#[serde(rename = "reqServingNode", default)]
		pub req_serving_node: bool,
		#[serde(rename = "reqTimeZone", default)]
		pub req_time_zone: bool,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&LocationInfoRequest> for LocationInfoRequest {
		fn from(value: &LocationInfoRequest) -> Self {
			value.clone()
		}
	}

	/// Represents the requested location information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the requested location information.",
	///  "type": "object",
	///  "properties": {
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "currentLoc": {
	///      "type": "boolean"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "geoInfo": {
	///      "$ref": "#/components/schemas/GeographicArea"
	///    },
	///    "locationAge": {
	///      "$ref": "#/components/schemas/AgeOfLocationEstimate"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "smsfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "timezone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "vPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationInfoResult {
		#[serde(
			rename = "amfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "currentLoc",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub current_loc: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(rename = "geoInfo", default, skip_serializing_if = "Option::is_none")]
		pub geo_info: Option<GeographicArea>,
		#[serde(
			rename = "locationAge",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_age: Option<AgeOfLocationEstimate>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "smsfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tai: Option<Tai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub timezone: Option<TimeZone>,
		#[serde(rename = "vPlmnId", default, skip_serializing_if = "Option::is_none")]
		pub v_plmn_id: Option<PlmnId>,
	}

	impl From<&LocationInfoResult> for LocationInfoResult {
		fn from(value: &LocationInfoResult) -> Self {
			value.clone()
		}
	}

	/// LocationPrivacyInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_DISALLOWED",
	///    "LOCATION_ALLOWED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum LocationPrivacyInd {
		#[default]
		#[serde(rename = "LOCATION_DISALLOWED")]
		LocationDisallowed,
		#[serde(rename = "LOCATION_ALLOWED")]
		LocationAllowed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationPrivacyInd> for LocationPrivacyInd {
		fn from(value: &LocationPrivacyInd) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationPrivacyInd {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationDisallowed => "LOCATION_DISALLOWED".to_string(),
				Self::LocationAllowed => "LOCATION_ALLOWED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationPrivacyInd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_DISALLOWED" => Ok(Self::LocationDisallowed),
				"LOCATION_ALLOWED" => Ok(Self::LocationAllowed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationPrivacyInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationPrivacyInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationPrivacyInd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LocationReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "location"
	///  ],
	///  "properties": {
	///    "location": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationReport {
		pub location: UserLocation,
	}

	impl From<&LocationReport> for LocationReport {
		fn from(value: &LocationReport) -> Self {
			value.clone()
		}
	}

	/// LocationReportingConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "currentLocation"
	///  ],
	///  "properties": {
	///    "accuracy": {
	///      "$ref": "#/components/schemas/LocationAccuracy"
	///    },
	///    "currentLocation": {
	///      "type": "boolean"
	///    },
	///    "n3gppAccuracy": {
	///      "$ref": "#/components/schemas/LocationAccuracy"
	///    },
	///    "oneTime": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationReportingConfiguration {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub accuracy: Option<LocationAccuracy>,
		#[serde(rename = "currentLocation")]
		pub current_location: bool,
		#[serde(
			rename = "n3gppAccuracy",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n3gpp_accuracy: Option<LocationAccuracy>,
		#[serde(rename = "oneTime", default, skip_serializing_if = "Option::is_none")]
		pub one_time: Option<bool>,
	}

	impl From<&LocationReportingConfiguration> for LocationReportingConfiguration {
		fn from(value: &LocationReportingConfiguration) -> Self {
			value.clone()
		}
	}

	/// LossConnectivityCfg
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "maxDetectionTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LossConnectivityCfg {
		#[serde(
			rename = "maxDetectionTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_detection_time: Option<DurationSec>,
	}

	impl From<&LossConnectivityCfg> for LossConnectivityCfg {
		fn from(value: &LossConnectivityCfg) -> Self {
			value.clone()
		}
	}

	/// LossConnectivityReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "lossOfConnectReason"
	///  ],
	///  "properties": {
	///    "lossOfConnectReason": {
	///      "$ref": "#/components/schemas/LossOfConnectivityReason"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LossConnectivityReport {
		#[serde(rename = "lossOfConnectReason")]
		pub loss_of_connect_reason: LossOfConnectivityReason,
	}

	impl From<&LossConnectivityReport> for LossConnectivityReport {
		fn from(value: &LossConnectivityReport) -> Self {
			value.clone()
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

	/// Lpi
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "locationPrivacyInd"
	///  ],
	///  "properties": {
	///    "locationPrivacyInd": {
	///      "$ref": "#/components/schemas/LocationPrivacyInd"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Lpi {
		#[serde(rename = "locationPrivacyInd")]
		pub location_privacy_ind: LocationPrivacyInd,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&Lpi> for Lpi {
		fn from(value: &Lpi) -> Self {
			value.clone()
		}
	}

	/// MaxNumOfReports
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
	pub struct MaxNumOfReports(pub i64);

	impl ::std::ops::Deref for MaxNumOfReports {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<MaxNumOfReports> for i64 {
		fn from(value: MaxNumOfReports) -> Self {
			value.0
		}
	}

	impl From<&MaxNumOfReports> for MaxNumOfReports {
		fn from(value: &MaxNumOfReports) -> Self {
			value.clone()
		}
	}

	impl From<i64> for MaxNumOfReports {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MaxNumOfReports {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MaxNumOfReports {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaxNumOfReports {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaxNumOfReports {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MaxNumOfReports {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the 5MBS Subscription Data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the 5MBS Subscription Data.",
	///  "type": "object",
	///  "properties": {
	///    "mbsAllowed": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mbsSessionIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsSessionId"
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
	pub struct MbsSubscriptionData {
		#[serde(rename = "mbsAllowed", default)]
		pub mbs_allowed: bool,
		#[serde(
			rename = "mbsSessionIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbs_session_id_list: Vec<MbsSessionId>,
	}

	impl From<&MbsSubscriptionData> for MbsSubscriptionData {
		fn from(value: &MbsSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// McsPriorityIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct McsPriorityIndicator(pub bool);

	impl ::std::ops::Deref for McsPriorityIndicator {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<McsPriorityIndicator> for bool {
		fn from(value: McsPriorityIndicator) -> Self {
			value.0
		}
	}

	impl From<&McsPriorityIndicator> for McsPriorityIndicator {
		fn from(value: &McsPriorityIndicator) -> Self {
			value.clone()
		}
	}

	impl From<bool> for McsPriorityIndicator {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for McsPriorityIndicator {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for McsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for McsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for McsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for McsPriorityIndicator {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// MdtUserConsent
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CONSENT_NOT_GIVEN",
	///    "CONSENT_GIVEN"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum MdtUserConsent {
		#[default]
		#[serde(rename = "CONSENT_NOT_GIVEN")]
		ConsentNotGiven,
		#[serde(rename = "CONSENT_GIVEN")]
		ConsentGiven,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MdtUserConsent> for MdtUserConsent {
		fn from(value: &MdtUserConsent) -> Self {
			value.clone()
		}
	}

	impl ToString for MdtUserConsent {
		fn to_string(&self) -> String {
			match *self {
				Self::ConsentNotGiven => "CONSENT_NOT_GIVEN".to_string(),
				Self::ConsentGiven => "CONSENT_GIVEN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MdtUserConsent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CONSENT_NOT_GIVEN" => Ok(Self::ConsentNotGiven),
				"CONSENT_GIVEN" => Ok(Self::ConsentGiven),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MdtUserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MdtUserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MdtUserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MicoAllowed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MicoAllowed(pub bool);

	impl ::std::ops::Deref for MicoAllowed {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<MicoAllowed> for bool {
		fn from(value: MicoAllowed) -> Self {
			value.0
		}
	}

	impl From<&MicoAllowed> for MicoAllowed {
		fn from(value: &MicoAllowed) -> Self {
			value.clone()
		}
	}

	impl From<bool> for MicoAllowed {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MicoAllowed {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MicoAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MicoAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MicoAllowed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MicoAllowed {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// ModificationNotification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "notifyItems"
	///  ],
	///  "properties": {
	///    "notifyItems": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NotifyItem"
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
	pub struct ModificationNotification {
		#[serde(rename = "notifyItems")]
		pub notify_items: Vec<NotifyItem>,
	}

	impl From<&ModificationNotification> for ModificationNotification {
		fn from(value: &ModificationNotification) -> Self {
			value.clone()
		}
	}

	/// MonitoringConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "eventType"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "associationType": {
	///      "$ref": "#/components/schemas/AssociationType"
	///    },
	///    "datalinkReportCfg": {
	///      "$ref": "#/components/schemas/DatalinkReportingConfiguration"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "eventType": {
	///      "$ref": "#/components/schemas/EventType"
	///    },
	///    "idleStatusInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "immediateFlag": {
	///      "type": "boolean"
	///    },
	///    "locationReportingConfiguration": {
	///      "$ref": "#/components/schemas/LocationReportingConfiguration"
	///    },
	///    "lossConnectivityCfg": {
	///      "$ref": "#/components/schemas/LossConnectivityCfg"
	///    },
	///    "maximumLatency": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "maximumResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "pduSessionStatusCfg": {
	///      "$ref": "#/components/schemas/PduSessionStatusCfg"
	///    },
	///    "reachabilityForDataCfg": {
	///      "$ref": "#/components/schemas/ReachabilityForDataConfiguration"
	///    },
	///    "reachabilityForSmsCfg": {
	///      "$ref": "#/components/schemas/ReachabilityForSmsConfiguration"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "suggestedPacketNumDl": {
	///      "type": "integer",
	///      "minimum": 1.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MonitoringConfiguration {
		#[serde(rename = "afId", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<String>,
		#[serde(
			rename = "associationType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub association_type: Option<AssociationType>,
		#[serde(
			rename = "datalinkReportCfg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub datalink_report_cfg: Option<DatalinkReportingConfiguration>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "eventType")]
		pub event_type: EventType,
		#[serde(rename = "idleStatusInd", default)]
		pub idle_status_ind: bool,
		#[serde(
			rename = "immediateFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub immediate_flag: Option<bool>,
		#[serde(
			rename = "locationReportingConfiguration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_reporting_configuration: Option<LocationReportingConfiguration>,
		#[serde(
			rename = "lossConnectivityCfg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub loss_connectivity_cfg: Option<LossConnectivityCfg>,
		#[serde(
			rename = "maximumLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_latency: Option<DurationSec>,
		#[serde(
			rename = "maximumResponseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_response_time: Option<DurationSec>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(
			rename = "pduSessionStatusCfg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_status_cfg: Option<PduSessionStatusCfg>,
		#[serde(
			rename = "reachabilityForDataCfg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_for_data_cfg: Option<ReachabilityForDataConfiguration>,
		#[serde(
			rename = "reachabilityForSmsCfg",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_for_sms_cfg: Option<ReachabilityForSmsConfiguration>,
		#[serde(
			rename = "singleNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub single_nssai: Option<Snssai>,
		#[serde(
			rename = "suggestedPacketNumDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub suggested_packet_num_dl: Option<std::num::NonZeroU64>,
	}

	impl From<&MonitoringConfiguration> for MonitoringConfiguration {
		fn from(value: &MonitoringConfiguration) -> Self {
			value.clone()
		}
	}

	/// MonitoringEvent
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "eventType"
	///  ],
	///  "properties": {
	///    "eventType": {
	///      "$ref": "#/components/schemas/EventType"
	///    },
	///    "revokedCause": {
	///      "$ref": "#/components/schemas/RevokedCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MonitoringEvent {
		#[serde(rename = "eventType")]
		pub event_type: EventType,
		#[serde(
			rename = "revokedCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub revoked_cause: Option<RevokedCause>,
	}

	impl From<&MonitoringEvent> for MonitoringEvent {
		fn from(value: &MonitoringEvent) -> Self {
			value.clone()
		}
	}

	/// MonitoringReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "eventType",
	///    "referenceId",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "eventType": {
	///      "$ref": "#/components/schemas/EventType"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "reachabilityForSmsReport": {
	///      "$ref": "#/components/schemas/ReachabilityForSmsReport"
	///    },
	///    "reachabilityReport": {
	///      "$ref": "#/components/schemas/ReachabilityReport"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "report": {
	///      "$ref": "#/components/schemas/Report"
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
	pub struct MonitoringReport {
		#[serde(rename = "eventType")]
		pub event_type: EventType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "reachabilityForSmsReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_for_sms_report: Option<ReachabilityForSmsReport>,
		#[serde(
			rename = "reachabilityReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_report: Option<ReachabilityReport>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub report: Option<Report>,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
	}

	impl From<&MonitoringReport> for MonitoringReport {
		fn from(value: &MonitoringReport) -> Self {
			value.clone()
		}
	}

	/// MpsPriorityIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MpsPriorityIndicator(pub bool);

	impl ::std::ops::Deref for MpsPriorityIndicator {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<MpsPriorityIndicator> for bool {
		fn from(value: MpsPriorityIndicator) -> Self {
			value.0
		}
	}

	impl From<&MpsPriorityIndicator> for MpsPriorityIndicator {
		fn from(value: &MpsPriorityIndicator) -> Self {
			value.clone()
		}
	}

	impl From<bool> for MpsPriorityIndicator {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MpsPriorityIndicator {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MpsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MpsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MpsPriorityIndicator {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MpsPriorityIndicator {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// MulticastMbsGroupMemb
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "multicastGroupMemb"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "internalGroupIdentifier": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "multicastGroupMemb": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
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
	pub struct MulticastMbsGroupMemb {
		#[serde(
			rename = "afInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_instance_id: Option<String>,
		#[serde(
			rename = "internalGroupIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub internal_group_identifier: Option<GroupId>,
		#[serde(rename = "multicastGroupMemb")]
		pub multicast_group_memb: Vec<Gpsi>,
	}

	impl From<&MulticastMbsGroupMemb> for MulticastMbsGroupMemb {
		fn from(value: &MulticastMbsGroupMemb) -> Self {
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

	/// NbIoTUePriority
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
	pub struct NbIoTUePriority(pub u8);

	impl ::std::ops::Deref for NbIoTUePriority {
		type Target = u8;
		fn deref(&self) -> &u8 {
			&self.0
		}
	}

	impl From<NbIoTUePriority> for u8 {
		fn from(value: NbIoTUePriority) -> Self {
			value.0
		}
	}

	impl From<&NbIoTUePriority> for NbIoTUePriority {
		fn from(value: &NbIoTUePriority) -> Self {
			value.clone()
		}
	}

	impl From<u8> for NbIoTUePriority {
		fn from(value: u8) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NbIoTUePriority {
		type Err = <u8 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for NbIoTUePriority {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NbIoTUePriority {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NbIoTUePriority {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for NbIoTUePriority {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Represents NIDD authorization update information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents NIDD authorization update information.",
	///  "type": "object",
	///  "required": [
	///    "authorizationData"
	///  ],
	///  "properties": {
	///    "authorizationData": {
	///      "$ref": "#/components/schemas/AuthorizationData"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "invalidityInd": {
	///      "type": "boolean"
	///    },
	///    "niddCause": {
	///      "$ref": "#/components/schemas/NiddCause"
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
	pub struct NiddAuthUpdateInfo {
		#[serde(rename = "authorizationData")]
		pub authorization_data: AuthorizationData,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "invalidityInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invalidity_ind: Option<bool>,
		#[serde(rename = "niddCause", default, skip_serializing_if = "Option::is_none")]
		pub nidd_cause: Option<NiddCause>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&NiddAuthUpdateInfo> for NiddAuthUpdateInfo {
		fn from(value: &NiddAuthUpdateInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a NIDD authorization update notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a NIDD authorization update notification.",
	///  "type": "object",
	///  "required": [
	///    "niddAuthUpdateInfoList"
	///  ],
	///  "properties": {
	///    "niddAuthUpdateInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NiddAuthUpdateInfo"
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
	pub struct NiddAuthUpdateNotification {
		#[serde(rename = "niddAuthUpdateInfoList")]
		pub nidd_auth_update_info_list: Vec<NiddAuthUpdateInfo>,
	}

	impl From<&NiddAuthUpdateNotification> for NiddAuthUpdateNotification {
		fn from(value: &NiddAuthUpdateNotification) -> Self {
			value.clone()
		}
	}

	/// NiddCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SUBSCRIPTION_WITHDRAWAL",
	///    "DNN_REMOVED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum NiddCause {
		#[default]
		#[serde(rename = "SUBSCRIPTION_WITHDRAWAL")]
		SubscriptionWithdrawal,
		#[serde(rename = "DNN_REMOVED")]
		DnnRemoved,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NiddCause> for NiddCause {
		fn from(value: &NiddCause) -> Self {
			value.clone()
		}
	}

	impl ToString for NiddCause {
		fn to_string(&self) -> String {
			match *self {
				Self::SubscriptionWithdrawal => "SUBSCRIPTION_WITHDRAWAL".to_string(),
				Self::DnnRemoved => "DNN_REMOVED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NiddCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUBSCRIPTION_WITHDRAWAL" => Ok(Self::SubscriptionWithdrawal),
				"DNN_REMOVED" => Ok(Self::DnnRemoved),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NiddCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NiddCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NiddCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NiddInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afId"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "extGroupId": {
	///      "$ref": "#/components/schemas/ExternalGroupId"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NiddInformation {
		#[serde(rename = "afId")]
		pub af_id: String,
		#[serde(
			rename = "extGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_group_id: Option<ExternalGroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
	}

	impl From<&NiddInformation> for NiddInformation {
		fn from(value: &NiddInformation) -> Self {
			value.clone()
		}
	}

	/// NodeType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "AUSF",
	///    "VLR",
	///    "SGSN",
	///    "S_CSCF",
	///    "BSF",
	///    "GAN_AAA_SERVER",
	///    "WLAN_AAA_SERVER",
	///    "MME"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum NodeType {
		#[default]
		#[serde(rename = "AUSF")]
		Ausf,
		#[serde(rename = "VLR")]
		Vlr,
		#[serde(rename = "SGSN")]
		Sgsn,
		#[serde(rename = "S_CSCF")]
		SCscf,
		#[serde(rename = "BSF")]
		Bsf,
		#[serde(rename = "GAN_AAA_SERVER")]
		GanAaaServer,
		#[serde(rename = "WLAN_AAA_SERVER")]
		WlanAaaServer,
		#[serde(rename = "MME")]
		Mme,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NodeType> for NodeType {
		fn from(value: &NodeType) -> Self {
			value.clone()
		}
	}

	impl ToString for NodeType {
		fn to_string(&self) -> String {
			match *self {
				Self::Ausf => "AUSF".to_string(),
				Self::Vlr => "VLR".to_string(),
				Self::Sgsn => "SGSN".to_string(),
				Self::SCscf => "S_CSCF".to_string(),
				Self::Bsf => "BSF".to_string(),
				Self::GanAaaServer => "GAN_AAA_SERVER".to_string(),
				Self::WlanAaaServer => "WLAN_AAA_SERVER".to_string(),
				Self::Mme => "MME".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NodeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AUSF" => Ok(Self::Ausf),
				"VLR" => Ok(Self::Vlr),
				"SGSN" => Ok(Self::Sgsn),
				"S_CSCF" => Ok(Self::SCscf),
				"BSF" => Ok(Self::Bsf),
				"GAN_AAA_SERVER" => Ok(Self::GanAaaServer),
				"WLAN_AAA_SERVER" => Ok(Self::WlanAaaServer),
				"MME" => Ok(Self::Mme),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Nssai
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "defaultSingleNssais"
	///  ],
	///  "properties": {
	///    "additionalSnssaiData": {
	///      "description": "A map(list of key-value pairs) where singleNssai
	/// serves as key of AdditionalSnssaiData",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/AdditionalSnssaiData"
	///      }
	///    },
	///    "defaultSingleNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "singleNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "suppressNssrgInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Nssai(pub Option<NssaiInner>);

	impl ::std::ops::Deref for Nssai {
		type Target = Option<NssaiInner>;
		fn deref(&self) -> &Option<NssaiInner> {
			&self.0
		}
	}

	impl From<Nssai> for Option<NssaiInner> {
		fn from(value: Nssai) -> Self {
			value.0
		}
	}

	impl From<&Nssai> for Nssai {
		fn from(value: &Nssai) -> Self {
			value.clone()
		}
	}

	impl From<Option<NssaiInner>> for Nssai {
		fn from(value: Option<NssaiInner>) -> Self {
			Self(value)
		}
	}

	/// NssaiInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "defaultSingleNssais"
	///  ],
	///  "properties": {
	///    "additionalSnssaiData": {
	///      "description": "A map(list of key-value pairs) where singleNssai
	/// serves as key of AdditionalSnssaiData",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/AdditionalSnssaiData"
	///      }
	///    },
	///    "defaultSingleNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "singleNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "suppressNssrgInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssaiInner {
		/// A map(list of key-value pairs) where singleNssai serves as key of
		/// AdditionalSnssaiData
		#[serde(
			rename = "additionalSnssaiData",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub additional_snssai_data: ::std::collections::HashMap<String, AdditionalSnssaiData>,
		#[serde(rename = "defaultSingleNssais")]
		pub default_single_nssais: Vec<Snssai>,
		#[serde(
			rename = "provisioningTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub provisioning_time: Option<DateTime>,
		#[serde(
			rename = "singleNssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub single_nssais: Vec<Snssai>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "suppressNssrgInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub suppress_nssrg_ind: Option<bool>,
	}

	impl From<&NssaiInner> for NssaiInner {
		fn from(value: &NssaiInner) -> Self {
			value.clone()
		}
	}

	/// NumOfRequestedVectors
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "integer",
	///  "maximum": 5.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NumOfRequestedVectors(pub i64);

	impl ::std::ops::Deref for NumOfRequestedVectors {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<NumOfRequestedVectors> for i64 {
		fn from(value: NumOfRequestedVectors) -> Self {
			value.0
		}
	}

	impl From<&NumOfRequestedVectors> for NumOfRequestedVectors {
		fn from(value: &NumOfRequestedVectors) -> Self {
			value.clone()
		}
	}

	impl From<i64> for NumOfRequestedVectors {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NumOfRequestedVectors {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for NumOfRequestedVectors {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NumOfRequestedVectors {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NumOfRequestedVectors {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for NumOfRequestedVectors {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// The complete set of information relevant to an NWDAF serving the UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The complete set of information relevant to an NWDAF
	/// serving the UE",
	///  "type": "object",
	///  "required": [
	///    "analyticsIds",
	///    "nwdafInstanceId"
	///  ],
	///  "properties": {
	///    "analyticsIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventId"
	///      },
	///      "minItems": 1
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "nwdafInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nwdafSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
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
	pub struct NwdafRegistration {
		#[serde(rename = "analyticsIds")]
		pub analytics_ids: Vec<EventId>,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(rename = "nwdafInstanceId")]
		pub nwdaf_instance_id: NfInstanceId,
		#[serde(
			rename = "nwdafSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_set_id: Option<NfSetId>,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NwdafRegistration> for NwdafRegistration {
		fn from(value: &NwdafRegistration) -> Self {
			value.clone()
		}
	}

	/// List of NwdafRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of NwdafRegistration",
	///  "type": "object",
	///  "required": [
	///    "nwdafRegistrationList"
	///  ],
	///  "properties": {
	///    "nwdafRegistrationList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafRegistration"
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
	pub struct NwdafRegistrationInfo {
		#[serde(rename = "nwdafRegistrationList")]
		pub nwdaf_registration_list: Vec<NwdafRegistration>,
	}

	impl From<&NwdafRegistrationInfo> for NwdafRegistrationInfo {
		fn from(value: &NwdafRegistrationInfo) -> Self {
			value.clone()
		}
	}

	/// Contains attributes of NwdafRegistration that can be modified using
	/// PATCH
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains attributes of NwdafRegistration that can be
	/// modified using PATCH",
	///  "type": "object",
	///  "required": [
	///    "nwdafInstanceId"
	///  ],
	///  "properties": {
	///    "analyticsIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventId"
	///      }
	///    },
	///    "nwdafInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nwdafSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
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
	pub struct NwdafRegistrationModification {
		#[serde(
			rename = "analyticsIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub analytics_ids: Vec<EventId>,
		#[serde(rename = "nwdafInstanceId")]
		pub nwdaf_instance_id: NfInstanceId,
		#[serde(
			rename = "nwdafSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_set_id: Option<NfSetId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NwdafRegistrationModification> for NwdafRegistrationModification {
		fn from(value: &NwdafRegistrationModification) -> Self {
			value.clone()
		}
	}

	/// OperationMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "WB_S1",
	///    "NB_S1",
	///    "WB_N1",
	///    "NB_N1",
	///    "NR_N1"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum OperationMode {
		#[default]
		#[serde(rename = "WB_S1")]
		WbS1,
		#[serde(rename = "NB_S1")]
		NbS1,
		#[serde(rename = "WB_N1")]
		WbN1,
		#[serde(rename = "NB_N1")]
		NbN1,
		#[serde(rename = "NR_N1")]
		NrN1,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OperationMode> for OperationMode {
		fn from(value: &OperationMode) -> Self {
			value.clone()
		}
	}

	impl ToString for OperationMode {
		fn to_string(&self) -> String {
			match *self {
				Self::WbS1 => "WB_S1".to_string(),
				Self::NbS1 => "NB_S1".to_string(),
				Self::WbN1 => "WB_N1".to_string(),
				Self::NbN1 => "NB_N1".to_string(),
				Self::NrN1 => "NR_N1".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OperationMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"WB_S1" => Ok(Self::WbS1),
				"NB_S1" => Ok(Self::NbS1),
				"WB_N1" => Ok(Self::WbN1),
				"NB_N1" => Ok(Self::NbN1),
				"NR_N1" => Ok(Self::NrN1),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OperationMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OperationMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OperationMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the Operating System of the served UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Operating System of the served UE.",
	///  "type": "string",
	///  "format": "uuid"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct OsId(pub uuid::Uuid);

	impl ::std::ops::Deref for OsId {
		type Target = uuid::Uuid;
		fn deref(&self) -> &uuid::Uuid {
			&self.0
		}
	}

	impl From<OsId> for uuid::Uuid {
		fn from(value: OsId) -> Self {
			value.0
		}
	}

	impl From<&OsId> for OsId {
		fn from(value: &OsId) -> Self {
			value.clone()
		}
	}

	impl From<uuid::Uuid> for OsId {
		fn from(value: uuid::Uuid) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for OsId {
		type Err = <uuid::Uuid as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for OsId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OsId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OsId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for OsId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// PcfSelectionAssistanceInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "singleNssai"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PcfSelectionAssistanceInfo {
		pub dnn: Dnn,
		#[serde(rename = "singleNssai")]
		pub single_nssai: Snssai,
	}

	impl From<&PcfSelectionAssistanceInfo> for PcfSelectionAssistanceInfo {
		fn from(value: &PcfSelectionAssistanceInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the addressing information (IP addresses and/or FQDN) of the
	/// P-CSCF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the addressing information (IP addresses
	/// and/or FQDN) of the P-CSCF",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ipv4Addrs"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Addrs"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "fqdn"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "fqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
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
	#[serde(untagged)]
	pub enum PcscfAddress {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addrs")]
			ipv4_addrs: Vec<Ipv4Addr>,
		},
		Variant1 {
			#[serde(rename = "ipv6Addrs")]
			ipv6_addrs: Vec<Ipv6Addr>,
		},
		Variant2 {
			fqdn: Fqdn,
		},
	}

	impl From<&PcscfAddress> for PcscfAddress {
		fn from(value: &PcscfAddress) -> Self {
			value.clone()
		}
	}

	/// PcscfRestorationNotification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "failedPcscf": {
	///      "$ref": "#/components/schemas/PcscfAddress"
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
	pub struct PcscfRestorationNotification {
		#[serde(
			rename = "failedPcscf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failed_pcscf: Option<PcscfAddress>,
		pub supi: Supi,
	}

	impl From<&PcscfRestorationNotification> for PcscfRestorationNotification {
		fn from(value: &PcscfRestorationNotification) -> Self {
			value.clone()
		}
	}

	/// PdnConnectivityStatReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "pdnConnStat"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
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
	///    "pdnConnStat": {
	///      "$ref": "#/components/schemas/PdnConnectivityStatus"
	///    },
	///    "pduSeId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PdnConnectivityStatReport {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
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
		#[serde(rename = "pdnConnStat")]
		pub pdn_conn_stat: PdnConnectivityStatus,
		#[serde(rename = "pduSeId", default, skip_serializing_if = "Option::is_none")]
		pub pdu_se_id: Option<PduSessionId>,
		#[serde(
			rename = "pduSessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_sess_type: Option<PduSessionType>,
	}

	impl From<&PdnConnectivityStatReport> for PdnConnectivityStatReport {
		fn from(value: &PdnConnectivityStatReport) -> Self {
			value.clone()
		}
	}

	/// PdnConnectivityStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ESTABLISHED",
	///    "RELEASED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PdnConnectivityStatus {
		#[default]
		#[serde(rename = "ESTABLISHED")]
		Established,
		#[serde(rename = "RELEASED")]
		Released,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PdnConnectivityStatus> for PdnConnectivityStatus {
		fn from(value: &PdnConnectivityStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for PdnConnectivityStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Established => "ESTABLISHED".to_string(),
				Self::Released => "RELEASED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PdnConnectivityStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ESTABLISHED" => Ok(Self::Established),
				"RELEASED" => Ok(Self::Released),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PdnConnectivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PdnConnectivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PdnConnectivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PduSession
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "plmnId",
	///    "smfInstanceId"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
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
	pub struct PduSession {
		pub dnn: Dnn,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(
			rename = "singleNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub single_nssai: Option<Snssai>,
		#[serde(rename = "smfInstanceId")]
		pub smf_instance_id: NfInstanceId,
	}

	impl From<&PduSession> for PduSession {
		fn from(value: &PduSession) -> Self {
			value.clone()
		}
	}

	/// PduSessionContinuityInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "MAINTAIN_PDUSESSION",
	///    "RECONNECT_PDUSESSION",
	///    "RELEASE_PDUSESSION"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PduSessionContinuityInd {
		#[default]
		#[serde(rename = "MAINTAIN_PDUSESSION")]
		MaintainPdusession,
		#[serde(rename = "RECONNECT_PDUSESSION")]
		ReconnectPdusession,
		#[serde(rename = "RELEASE_PDUSESSION")]
		ReleasePdusession,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PduSessionContinuityInd> for PduSessionContinuityInd {
		fn from(value: &PduSessionContinuityInd) -> Self {
			value.clone()
		}
	}

	impl ToString for PduSessionContinuityInd {
		fn to_string(&self) -> String {
			match *self {
				Self::MaintainPdusession => "MAINTAIN_PDUSESSION".to_string(),
				Self::ReconnectPdusession => "RECONNECT_PDUSESSION".to_string(),
				Self::ReleasePdusession => "RELEASE_PDUSESSION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionContinuityInd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MAINTAIN_PDUSESSION" => Ok(Self::MaintainPdusession),
				"RECONNECT_PDUSESSION" => Ok(Self::ReconnectPdusession),
				"RELEASE_PDUSESSION" => Ok(Self::ReleasePdusession),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionContinuityInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionContinuityInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionContinuityInd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PduSessionStatusCfg
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionStatusCfg {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
	}

	impl From<&PduSessionStatusCfg> for PduSessionStatusCfg {
		fn from(value: &PduSessionStatusCfg) -> Self {
			value.clone()
		}
	}

	/// PduSessionTypes
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "allowedSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
	///      },
	///      "minItems": 1
	///    },
	///    "defaultSessionType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionTypes {
		#[serde(
			rename = "allowedSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_session_types: Vec<PduSessionType>,
		#[serde(
			rename = "defaultSessionType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub default_session_type: Option<PduSessionType>,
	}

	impl From<&PduSessionTypes> for PduSessionTypes {
		fn from(value: &PduSessionTypes) -> Self {
			value.clone()
		}
	}

	/// PeiUpdateInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "pei"
	///  ],
	///  "properties": {
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PeiUpdateInfo {
		pub pei: Pei,
	}

	impl From<&PeiUpdateInfo> for PeiUpdateInfo {
		fn from(value: &PeiUpdateInfo) -> Self {
			value.clone()
		}
	}

	/// PgwInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "pgwFqdn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "epdgInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
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
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PgwInfo {
		pub dnn: Dnn,
		#[serde(rename = "epdgInd", default)]
		pub epdg_ind: bool,
		#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_id: Option<NfInstanceId>,
		#[serde(rename = "pgwFqdn")]
		pub pgw_fqdn: Fqdn,
		#[serde(rename = "pgwIpAddr", default, skip_serializing_if = "Option::is_none")]
		pub pgw_ip_addr: Option<IpAddress>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
	}

	impl From<&PgwInfo> for PgwInfo {
		fn from(value: &PgwInfo) -> Self {
			value.clone()
		}
	}

	/// PlmnEcInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "ecRestrictionDataNb": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecRestrictionDataWb": {
	///      "$ref": "#/components/schemas/EcRestrictionDataWb"
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
	pub struct PlmnEcInfo {
		#[serde(rename = "ecRestrictionDataNb", default)]
		pub ec_restriction_data_nb: bool,
		#[serde(
			rename = "ecRestrictionDataWb",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ec_restriction_data_wb: Option<EcRestrictionDataWb>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&PlmnEcInfo> for PlmnEcInfo {
		fn from(value: &PlmnEcInfo) -> Self {
			value.clone()
		}
	}

	/// PlmnOperatorClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "lcsClientClass",
	///    "lcsClientIds"
	///  ],
	///  "properties": {
	///    "lcsClientClass": {
	///      "$ref": "#/components/schemas/LcsClientClass"
	///    },
	///    "lcsClientIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LcsClientId"
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
	pub struct PlmnOperatorClass {
		#[serde(rename = "lcsClientClass")]
		pub lcs_client_class: LcsClientClass,
		#[serde(rename = "lcsClientIds")]
		pub lcs_client_ids: Vec<LcsClientId>,
	}

	impl From<&PlmnOperatorClass> for PlmnOperatorClass {
		fn from(value: &PlmnOperatorClass) -> Self {
			value.clone()
		}
	}

	/// PlmnRestriction
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "coreNetworkTypeRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CoreNetworkType"
	///      }
	///    },
	///    "forbiddenAreas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Area"
	///      }
	///    },
	///    "primaryRatRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "ratRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "secondaryRatRestrictions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "uniqueItems": true
	///    },
	///    "serviceAreaRestriction": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PlmnRestriction {
		#[serde(
			rename = "coreNetworkTypeRestrictions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub core_network_type_restrictions: Vec<CoreNetworkType>,
		#[serde(
			rename = "forbiddenAreas",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forbidden_areas: Vec<Area>,
		#[serde(
			rename = "primaryRatRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub primary_rat_restrictions: Option<Vec<RatType>>,
		#[serde(
			rename = "ratRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rat_restrictions: Option<Vec<RatType>>,
		#[serde(
			rename = "secondaryRatRestrictions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_rat_restrictions: Option<Vec<RatType>>,
		#[serde(
			rename = "serviceAreaRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_area_restriction: Option<ServiceAreaRestriction>,
	}

	impl From<&PlmnRestriction> for PlmnRestriction {
		fn from(value: &PlmnRestriction) -> Self {
			value.clone()
		}
	}

	/// PpActiveTime
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "activeTime",
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "activeTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpActiveTime(pub Option<PpActiveTimeInner>);

	impl ::std::ops::Deref for PpActiveTime {
		type Target = Option<PpActiveTimeInner>;
		fn deref(&self) -> &Option<PpActiveTimeInner> {
			&self.0
		}
	}

	impl From<PpActiveTime> for Option<PpActiveTimeInner> {
		fn from(value: PpActiveTime) -> Self {
			value.0
		}
	}

	impl From<&PpActiveTime> for PpActiveTime {
		fn from(value: &PpActiveTime) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpActiveTimeInner>> for PpActiveTime {
		fn from(value: Option<PpActiveTimeInner>) -> Self {
			Self(value)
		}
	}

	/// PpActiveTimeInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "activeTime",
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "activeTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpActiveTimeInner {
		#[serde(rename = "activeTime")]
		pub active_time: DurationSec,
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpActiveTimeInner> for PpActiveTimeInner {
		fn from(value: &PpActiveTimeInner) -> Self {
			value.clone()
		}
	}

	/// PpData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "5mbsAuthorizationInfo": {
	///      "$ref": "#/components/schemas/5MbsAuthorizationInfo"
	///    },
	///    "acsInfo": {
	///      "$ref": "#/components/schemas/AcsInfoRm"
	///    },
	///    "communicationCharacteristics": {
	///      "$ref": "#/components/schemas/CommunicationCharacteristics"
	///    },
	///    "ecRestriction": {
	///      "$ref": "#/components/schemas/EcRestriction"
	///    },
	///    "expectedUeBehaviourParameters": {
	///      "$ref": "#/components/schemas/ExpectedUeBehaviour"
	///    },
	///    "lcsPrivacy": {
	///      "$ref": "#/components/schemas/LcsPrivacy"
	///    },
	///    "sorInfo": {
	///      "$ref": "#/components/schemas/schemas-SorInfo"
	///    },
	///    "stnSr": {
	///      "$ref": "#/components/schemas/StnSrRm"
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
	pub struct PpData(pub Option<PpDataInner>);

	impl ::std::ops::Deref for PpData {
		type Target = Option<PpDataInner>;
		fn deref(&self) -> &Option<PpDataInner> {
			&self.0
		}
	}

	impl From<PpData> for Option<PpDataInner> {
		fn from(value: PpData) -> Self {
			value.0
		}
	}

	impl From<&PpData> for PpData {
		fn from(value: &PpData) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpDataInner>> for PpData {
		fn from(value: Option<PpDataInner>) -> Self {
			Self(value)
		}
	}

	/// PpDataEntry
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "additionalEcsAddrConfigInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "communicationCharacteristics": {
	///      "$ref": "#/components/schemas/CommunicationCharacteristicsAF"
	///    },
	///    "ecRestriction": {
	///      "$ref": "#/components/schemas/EcRestriction"
	///    },
	///    "ecsAddrConfigInfo": {
	///      "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
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
	pub struct PpDataEntry(pub Option<PpDataEntryInner>);

	impl ::std::ops::Deref for PpDataEntry {
		type Target = Option<PpDataEntryInner>;
		fn deref(&self) -> &Option<PpDataEntryInner> {
			&self.0
		}
	}

	impl From<PpDataEntry> for Option<PpDataEntryInner> {
		fn from(value: PpDataEntry) -> Self {
			value.0
		}
	}

	impl From<&PpDataEntry> for PpDataEntry {
		fn from(value: &PpDataEntry) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpDataEntryInner>> for PpDataEntry {
		fn from(value: Option<PpDataEntryInner>) -> Self {
			Self(value)
		}
	}

	/// PpDataEntryInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "additionalEcsAddrConfigInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "communicationCharacteristics": {
	///      "$ref": "#/components/schemas/CommunicationCharacteristicsAF"
	///    },
	///    "ecRestriction": {
	///      "$ref": "#/components/schemas/EcRestriction"
	///    },
	///    "ecsAddrConfigInfo": {
	///      "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
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
	pub struct PpDataEntryInner {
		#[serde(
			rename = "additionalEcsAddrConfigInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_ecs_addr_config_infos: Vec<EcsAddrConfigInfo>,
		#[serde(
			rename = "communicationCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub communication_characteristics: Option<CommunicationCharacteristicsAf>,
		#[serde(
			rename = "ecRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ec_restriction: Option<EcRestriction>,
		#[serde(
			rename = "ecsAddrConfigInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ecs_addr_config_info: Option<EcsAddrConfigInfo>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(
			rename = "referenceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reference_id: Option<ReferenceId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpDataEntryInner> for PpDataEntryInner {
		fn from(value: &PpDataEntryInner) -> Self {
			value.clone()
		}
	}

	/// PpDataInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "5mbsAuthorizationInfo": {
	///      "$ref": "#/components/schemas/5MbsAuthorizationInfo"
	///    },
	///    "acsInfo": {
	///      "$ref": "#/components/schemas/AcsInfoRm"
	///    },
	///    "communicationCharacteristics": {
	///      "$ref": "#/components/schemas/CommunicationCharacteristics"
	///    },
	///    "ecRestriction": {
	///      "$ref": "#/components/schemas/EcRestriction"
	///    },
	///    "expectedUeBehaviourParameters": {
	///      "$ref": "#/components/schemas/ExpectedUeBehaviour"
	///    },
	///    "lcsPrivacy": {
	///      "$ref": "#/components/schemas/LcsPrivacy"
	///    },
	///    "sorInfo": {
	///      "$ref": "#/components/schemas/schemas-SorInfo"
	///    },
	///    "stnSr": {
	///      "$ref": "#/components/schemas/StnSrRm"
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
	pub struct PpDataInner {
		#[serde(rename = "acsInfo", default, skip_serializing_if = "Option::is_none")]
		pub acs_info: Option<AcsInfoRm>,
		#[serde(
			rename = "communicationCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub communication_characteristics: Option<CommunicationCharacteristics>,
		#[serde(
			rename = "ecRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ec_restriction: Option<EcRestriction>,
		#[serde(
			rename = "expectedUeBehaviourParameters",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expected_ue_behaviour_parameters: Option<ExpectedUeBehaviour>,
		#[serde(
			rename = "5mbsAuthorizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_mbs_authorization_info: Option<_5mbsAuthorizationInfo>,
		#[serde(
			rename = "lcsPrivacy",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_privacy: Option<LcsPrivacy>,
		#[serde(rename = "sorInfo", default, skip_serializing_if = "Option::is_none")]
		pub sor_info: Option<SchemasSorInfo>,
		#[serde(rename = "stnSr", default, skip_serializing_if = "Option::is_none")]
		pub stn_sr: Option<StnSrRm>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&PpDataInner> for PpDataInner {
		fn from(value: &PpDataInner) -> Self {
			value.clone()
		}
	}

	/// PpDlPacketCount
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct PpDlPacketCount(pub Option<i64>);

	impl ::std::ops::Deref for PpDlPacketCount {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<PpDlPacketCount> for Option<i64> {
		fn from(value: PpDlPacketCount) -> Self {
			value.0
		}
	}

	impl From<&PpDlPacketCount> for PpDlPacketCount {
		fn from(value: &PpDlPacketCount) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for PpDlPacketCount {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// PpDlPacketCountExt
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
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
	pub struct PpDlPacketCountExt(pub Option<PpDlPacketCountExtInner>);

	impl ::std::ops::Deref for PpDlPacketCountExt {
		type Target = Option<PpDlPacketCountExtInner>;
		fn deref(&self) -> &Option<PpDlPacketCountExtInner> {
			&self.0
		}
	}

	impl From<PpDlPacketCountExt> for Option<PpDlPacketCountExtInner> {
		fn from(value: PpDlPacketCountExt) -> Self {
			value.0
		}
	}

	impl From<&PpDlPacketCountExt> for PpDlPacketCountExt {
		fn from(value: &PpDlPacketCountExt) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpDlPacketCountExtInner>> for PpDlPacketCountExt {
		fn from(value: Option<PpDlPacketCountExtInner>) -> Self {
			Self(value)
		}
	}

	/// PpDlPacketCountExtInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
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
	pub struct PpDlPacketCountExtInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(
			rename = "singleNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub single_nssai: Option<Snssai>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpDlPacketCountExtInner> for PpDlPacketCountExtInner {
		fn from(value: &PpDlPacketCountExtInner) -> Self {
			value.clone()
		}
	}

	/// PpMaximumLatency
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "maximumLatency",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "maximumLatency": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpMaximumLatency(pub Option<PpMaximumLatencyInner>);

	impl ::std::ops::Deref for PpMaximumLatency {
		type Target = Option<PpMaximumLatencyInner>;
		fn deref(&self) -> &Option<PpMaximumLatencyInner> {
			&self.0
		}
	}

	impl From<PpMaximumLatency> for Option<PpMaximumLatencyInner> {
		fn from(value: PpMaximumLatency) -> Self {
			value.0
		}
	}

	impl From<&PpMaximumLatency> for PpMaximumLatency {
		fn from(value: &PpMaximumLatency) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpMaximumLatencyInner>> for PpMaximumLatency {
		fn from(value: Option<PpMaximumLatencyInner>) -> Self {
			Self(value)
		}
	}

	/// PpMaximumLatencyInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "maximumLatency",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "maximumLatency": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpMaximumLatencyInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(rename = "maximumLatency")]
		pub maximum_latency: DurationSec,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpMaximumLatencyInner> for PpMaximumLatencyInner {
		fn from(value: &PpMaximumLatencyInner) -> Self {
			value.clone()
		}
	}

	/// PpMaximumResponseTime
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "maximumResponseTime",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "maximumResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpMaximumResponseTime(pub Option<PpMaximumResponseTimeInner>);

	impl ::std::ops::Deref for PpMaximumResponseTime {
		type Target = Option<PpMaximumResponseTimeInner>;
		fn deref(&self) -> &Option<PpMaximumResponseTimeInner> {
			&self.0
		}
	}

	impl From<PpMaximumResponseTime> for Option<PpMaximumResponseTimeInner> {
		fn from(value: PpMaximumResponseTime) -> Self {
			value.0
		}
	}

	impl From<&PpMaximumResponseTime> for PpMaximumResponseTime {
		fn from(value: &PpMaximumResponseTime) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpMaximumResponseTimeInner>> for PpMaximumResponseTime {
		fn from(value: Option<PpMaximumResponseTimeInner>) -> Self {
			Self(value)
		}
	}

	/// PpMaximumResponseTimeInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "maximumResponseTime",
	///    "referenceId"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "maximumResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
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
	pub struct PpMaximumResponseTimeInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(rename = "maximumResponseTime")]
		pub maximum_response_time: DurationSec,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpMaximumResponseTimeInner> for PpMaximumResponseTimeInner {
		fn from(value: &PpMaximumResponseTimeInner) -> Self {
			value.clone()
		}
	}

	/// PpSubsRegTimer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "afInstanceId",
	///    "referenceId",
	///    "subsRegTimer"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "subsRegTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
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
	pub struct PpSubsRegTimer(pub Option<PpSubsRegTimerInner>);

	impl ::std::ops::Deref for PpSubsRegTimer {
		type Target = Option<PpSubsRegTimerInner>;
		fn deref(&self) -> &Option<PpSubsRegTimerInner> {
			&self.0
		}
	}

	impl From<PpSubsRegTimer> for Option<PpSubsRegTimerInner> {
		fn from(value: PpSubsRegTimer) -> Self {
			value.0
		}
	}

	impl From<&PpSubsRegTimer> for PpSubsRegTimer {
		fn from(value: &PpSubsRegTimer) -> Self {
			value.clone()
		}
	}

	impl From<Option<PpSubsRegTimerInner>> for PpSubsRegTimer {
		fn from(value: Option<PpSubsRegTimerInner>) -> Self {
			Self(value)
		}
	}

	/// PpSubsRegTimerInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "afInstanceId",
	///    "referenceId",
	///    "subsRegTimer"
	///  ],
	///  "properties": {
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "subsRegTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
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
	pub struct PpSubsRegTimerInner {
		#[serde(rename = "afInstanceId")]
		pub af_instance_id: String,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "referenceId")]
		pub reference_id: ReferenceId,
		#[serde(rename = "subsRegTimer")]
		pub subs_reg_timer: DurationSec,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&PpSubsRegTimerInner> for PpSubsRegTimerInner {
		fn from(value: &PpSubsRegTimerInner) -> Self {
			value.clone()
		}
	}

	/// PrivacyCheckRelatedAction
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_NOT_ALLOWED",
	///    "LOCATION_ALLOWED_WITH_NOTIFICATION",
	///    "LOCATION_ALLOWED_WITHOUT_NOTIFICATION",
	///    "LOCATION_ALLOWED_WITHOUT_RESPONSE",
	///    "LOCATION_RESTRICTED_WITHOUT_RESPONSE"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum PrivacyCheckRelatedAction {
		#[default]
		#[serde(rename = "LOCATION_NOT_ALLOWED")]
		LocationNotAllowed,
		#[serde(rename = "LOCATION_ALLOWED_WITH_NOTIFICATION")]
		LocationAllowedWithNotification,
		#[serde(rename = "LOCATION_ALLOWED_WITHOUT_NOTIFICATION")]
		LocationAllowedWithoutNotification,
		#[serde(rename = "LOCATION_ALLOWED_WITHOUT_RESPONSE")]
		LocationAllowedWithoutResponse,
		#[serde(rename = "LOCATION_RESTRICTED_WITHOUT_RESPONSE")]
		LocationRestrictedWithoutResponse,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PrivacyCheckRelatedAction> for PrivacyCheckRelatedAction {
		fn from(value: &PrivacyCheckRelatedAction) -> Self {
			value.clone()
		}
	}

	impl ToString for PrivacyCheckRelatedAction {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationNotAllowed => "LOCATION_NOT_ALLOWED".to_string(),
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
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PrivacyCheckRelatedAction {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_NOT_ALLOWED" => Ok(Self::LocationNotAllowed),
				"LOCATION_ALLOWED_WITH_NOTIFICATION" => Ok(Self::LocationAllowedWithNotification),
				"LOCATION_ALLOWED_WITHOUT_NOTIFICATION" => {
					Ok(Self::LocationAllowedWithoutNotification)
				}
				"LOCATION_ALLOWED_WITHOUT_RESPONSE" => Ok(Self::LocationAllowedWithoutResponse),
				"LOCATION_RESTRICTED_WITHOUT_RESPONSE" => {
					Ok(Self::LocationRestrictedWithoutResponse)
				}
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PrivacyCheckRelatedAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PrivacyCheckRelatedAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PrivacyCheckRelatedAction {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the PLMN identities where the Prose services are authorised to
	/// use and the authorised Prose services on this given PLMNs.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the PLMN identities where the Prose services
	/// are authorised to use and the authorised Prose services on this given
	/// PLMNs.",
	///  "type": "object",
	///  "required": [
	///    "visitedPlmn"
	///  ],
	///  "properties": {
	///    "proseDirectAllowed": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ProseDirectAllowed"
	///      },
	///      "minItems": 1
	///    },
	///    "visitedPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProSeAllowedPlmn {
		#[serde(
			rename = "proseDirectAllowed",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub prose_direct_allowed: Vec<ProseDirectAllowed>,
		#[serde(rename = "visitedPlmn")]
		pub visited_plmn: PlmnId,
	}

	impl From<&ProSeAllowedPlmn> for ProSeAllowedPlmn {
		fn from(value: &ProSeAllowedPlmn) -> Self {
			value.clone()
		}
	}

	/// ProSeAuthenticationInfoRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "relayServiceCode",
	///    "servingNetworkName"
	///  ],
	///  "properties": {
	///    "relayServiceCode": {
	///      "$ref": "#/components/schemas/RelayServiceCode"
	///    },
	///    "resynchronizationInfo": {
	///      "$ref": "#/components/schemas/ResynchronizationInfo"
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
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
	pub struct ProSeAuthenticationInfoRequest {
		#[serde(rename = "relayServiceCode")]
		pub relay_service_code: RelayServiceCode,
		#[serde(
			rename = "resynchronizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resynchronization_info: Option<ResynchronizationInfo>,
		#[serde(rename = "servingNetworkName")]
		pub serving_network_name: ServingNetworkName,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeAuthenticationInfoRequest> for ProSeAuthenticationInfoRequest {
		fn from(value: &ProSeAuthenticationInfoRequest) -> Self {
			value.clone()
		}
	}

	/// ProSeAuthenticationInfoResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "authType"
	///  ],
	///  "properties": {
	///    "authType": {
	///      "$ref": "#/components/schemas/AuthType"
	///    },
	///    "proseAuthenticationVectors": {
	///      "$ref": "#/components/schemas/ProSeAuthenticationVectors"
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
	pub struct ProSeAuthenticationInfoResult {
		#[serde(rename = "authType")]
		pub auth_type: AuthType,
		#[serde(
			rename = "proseAuthenticationVectors",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_authentication_vectors: Option<ProSeAuthenticationVectors>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeAuthenticationInfoResult> for ProSeAuthenticationInfoResult {
		fn from(value: &ProSeAuthenticationInfoResult) -> Self {
			value.clone()
		}
	}

	/// ProSeAuthenticationVectors
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AvEapAkaPrime"
	///      },
	///      "maxItems": 5,
	///      "minItems": 1
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProSeAuthenticationVectors(pub Vec<AvEapAkaPrime>);

	impl ::std::ops::Deref for ProSeAuthenticationVectors {
		type Target = Vec<AvEapAkaPrime>;
		fn deref(&self) -> &Vec<AvEapAkaPrime> {
			&self.0
		}
	}

	impl From<ProSeAuthenticationVectors> for Vec<AvEapAkaPrime> {
		fn from(value: ProSeAuthenticationVectors) -> Self {
			value.0
		}
	}

	impl From<&ProSeAuthenticationVectors> for ProSeAuthenticationVectors {
		fn from(value: &ProSeAuthenticationVectors) -> Self {
			value.clone()
		}
	}

	impl From<Vec<AvEapAkaPrime>> for ProSeAuthenticationVectors {
		fn from(value: Vec<AvEapAkaPrime>) -> Self {
			Self(value)
		}
	}

	/// Indicates the 5G ProSe Direct services that can be authorised to use in
	/// the given PLMN for the UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the 5G ProSe Direct services that can be
	/// authorised to use in the given PLMN for the UE.",
	///  "type": "string",
	///  "enum": [
	///    "ANNOUNCE",
	///    "MONITOR",
	///    "RESTRICTD_ANNOUNCE",
	///    "RESTRICTD_MONITOR",
	///    "DISCOVERER",
	///    "DISCOVEREE",
	///    "BROADCAST",
	///    "GROUPCAST",
	///    "UNICAST",
	///    "LAYER2_RELAY",
	///    "LAYER3_RELAY"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ProseDirectAllowed {
		#[default]
		#[serde(rename = "ANNOUNCE")]
		Announce,
		#[serde(rename = "MONITOR")]
		Monitor,
		#[serde(rename = "RESTRICTD_ANNOUNCE")]
		RestrictdAnnounce,
		#[serde(rename = "RESTRICTD_MONITOR")]
		RestrictdMonitor,
		#[serde(rename = "DISCOVERER")]
		Discoverer,
		#[serde(rename = "DISCOVEREE")]
		Discoveree,
		#[serde(rename = "BROADCAST")]
		Broadcast,
		#[serde(rename = "GROUPCAST")]
		Groupcast,
		#[serde(rename = "UNICAST")]
		Unicast,
		#[serde(rename = "LAYER2_RELAY")]
		Layer2Relay,
		#[serde(rename = "LAYER3_RELAY")]
		Layer3Relay,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ProseDirectAllowed> for ProseDirectAllowed {
		fn from(value: &ProseDirectAllowed) -> Self {
			value.clone()
		}
	}

	impl ToString for ProseDirectAllowed {
		fn to_string(&self) -> String {
			match *self {
				Self::Announce => "ANNOUNCE".to_string(),
				Self::Monitor => "MONITOR".to_string(),
				Self::RestrictdAnnounce => "RESTRICTD_ANNOUNCE".to_string(),
				Self::RestrictdMonitor => "RESTRICTD_MONITOR".to_string(),
				Self::Discoverer => "DISCOVERER".to_string(),
				Self::Discoveree => "DISCOVEREE".to_string(),
				Self::Broadcast => "BROADCAST".to_string(),
				Self::Groupcast => "GROUPCAST".to_string(),
				Self::Unicast => "UNICAST".to_string(),
				Self::Layer2Relay => "LAYER2_RELAY".to_string(),
				Self::Layer3Relay => "LAYER3_RELAY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ProseDirectAllowed {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ANNOUNCE" => Ok(Self::Announce),
				"MONITOR" => Ok(Self::Monitor),
				"RESTRICTD_ANNOUNCE" => Ok(Self::RestrictdAnnounce),
				"RESTRICTD_MONITOR" => Ok(Self::RestrictdMonitor),
				"DISCOVERER" => Ok(Self::Discoverer),
				"DISCOVEREE" => Ok(Self::Discoveree),
				"BROADCAST" => Ok(Self::Broadcast),
				"GROUPCAST" => Ok(Self::Groupcast),
				"UNICAST" => Ok(Self::Unicast),
				"LAYER2_RELAY" => Ok(Self::Layer2Relay),
				"LAYER3_RELAY" => Ok(Self::Layer3Relay),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ProseDirectAllowed {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProseDirectAllowed {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProseDirectAllowed {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the ProSe Subscription Data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the ProSe Subscription Data.",
	///  "type": "object",
	///  "properties": {
	///    "nrUePc5Ambr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "proseAllowedPlmn": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ProSeAllowedPlmn"
	///      },
	///      "minItems": 1
	///    },
	///    "proseServiceAuth": {
	///      "$ref": "#/components/schemas/ProseServiceAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProseSubscriptionData {
		#[serde(
			rename = "nrUePc5Ambr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_ue_pc5_ambr: Option<BitRate>,
		#[serde(
			rename = "proseAllowedPlmn",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub prose_allowed_plmn: Vec<ProSeAllowedPlmn>,
		#[serde(
			rename = "proseServiceAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_service_auth: Option<ProseServiceAuth>,
	}

	impl From<&ProseSubscriptionData> for ProseSubscriptionData {
		fn from(value: &ProseSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// PtwParameters
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "operationMode",
	///    "ptwValue"
	///  ],
	///  "properties": {
	///    "extendedPtwValue": {
	///      "type": "string",
	///      "pattern": "^([0-1]{8})$"
	///    },
	///    "operationMode": {
	///      "$ref": "#/components/schemas/OperationMode"
	///    },
	///    "ptwValue": {
	///      "type": "string",
	///      "pattern": "^([0-1]{4})$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PtwParameters {
		#[serde(
			rename = "extendedPtwValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub extended_ptw_value: Option<PtwParametersExtendedPtwValue>,
		#[serde(rename = "operationMode")]
		pub operation_mode: OperationMode,
		#[serde(rename = "ptwValue")]
		pub ptw_value: PtwParametersPtwValue,
	}

	impl From<&PtwParameters> for PtwParameters {
		fn from(value: &PtwParameters) -> Self {
			value.clone()
		}
	}

	/// PtwParametersExtendedPtwValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([0-1]{8})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct PtwParametersExtendedPtwValue(String);

	impl ::std::ops::Deref for PtwParametersExtendedPtwValue {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PtwParametersExtendedPtwValue> for String {
		fn from(value: PtwParametersExtendedPtwValue) -> Self {
			value.0
		}
	}

	impl From<&PtwParametersExtendedPtwValue> for PtwParametersExtendedPtwValue {
		fn from(value: &PtwParametersExtendedPtwValue) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PtwParametersExtendedPtwValue {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-1]{8})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([0-1]{8})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PtwParametersExtendedPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PtwParametersExtendedPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PtwParametersExtendedPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PtwParametersExtendedPtwValue {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// PtwParametersPtwValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([0-1]{4})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct PtwParametersPtwValue(String);

	impl ::std::ops::Deref for PtwParametersPtwValue {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PtwParametersPtwValue> for String {
		fn from(value: PtwParametersPtwValue) -> Self {
			value.0
		}
	}

	impl From<&PtwParametersPtwValue> for PtwParametersPtwValue {
		fn from(value: &PtwParametersPtwValue) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PtwParametersPtwValue {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-1]{4})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([0-1]{4})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PtwParametersPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PtwParametersPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PtwParametersPtwValue {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PtwParametersPtwValue {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// PurgeFlag
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PurgeFlag(pub bool);

	impl ::std::ops::Deref for PurgeFlag {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<PurgeFlag> for bool {
		fn from(value: PurgeFlag) -> Self {
			value.0
		}
	}

	impl From<&PurgeFlag> for PurgeFlag {
		fn from(value: &PurgeFlag) -> Self {
			value.clone()
		}
	}

	impl From<bool> for PurgeFlag {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PurgeFlag {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PurgeFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PurgeFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PurgeFlag {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PurgeFlag {
		fn to_string(&self) -> String {
			self.0.to_string()
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
		NewUnchecked
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

	/// ReachabilityForDataConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "reportCfg"
	///  ],
	///  "properties": {
	///    "minInterval": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "reportCfg": {
	///      "$ref": "#/components/schemas/ReachabilityForDataReportConfig"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReachabilityForDataConfiguration {
		#[serde(
			rename = "minInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_interval: Option<DurationSec>,
		#[serde(rename = "reportCfg")]
		pub report_cfg: ReachabilityForDataReportConfig,
	}

	impl From<&ReachabilityForDataConfiguration> for ReachabilityForDataConfiguration {
		fn from(value: &ReachabilityForDataConfiguration) -> Self {
			value.clone()
		}
	}

	/// ReachabilityForDataReportConfig
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DIRECT_REPORT",
	///    "INDIRECT_REPORT"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ReachabilityForDataReportConfig {
		#[default]
		#[serde(rename = "DIRECT_REPORT")]
		DirectReport,
		#[serde(rename = "INDIRECT_REPORT")]
		IndirectReport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReachabilityForDataReportConfig> for ReachabilityForDataReportConfig {
		fn from(value: &ReachabilityForDataReportConfig) -> Self {
			value.clone()
		}
	}

	impl ToString for ReachabilityForDataReportConfig {
		fn to_string(&self) -> String {
			match *self {
				Self::DirectReport => "DIRECT_REPORT".to_string(),
				Self::IndirectReport => "INDIRECT_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReachabilityForDataReportConfig {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DIRECT_REPORT" => Ok(Self::DirectReport),
				"INDIRECT_REPORT" => Ok(Self::IndirectReport),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReachabilityForDataReportConfig {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReachabilityForDataReportConfig {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReachabilityForDataReportConfig {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ReachabilityForSmsConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "REACHABILITY_FOR_SMS_OVER_NAS",
	///    "REACHABILITY_FOR_SMS_OVER_IP"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ReachabilityForSmsConfiguration {
		#[default]
		#[serde(rename = "REACHABILITY_FOR_SMS_OVER_NAS")]
		ReachabilityForSmsOverNas,
		#[serde(rename = "REACHABILITY_FOR_SMS_OVER_IP")]
		ReachabilityForSmsOverIp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReachabilityForSmsConfiguration> for ReachabilityForSmsConfiguration {
		fn from(value: &ReachabilityForSmsConfiguration) -> Self {
			value.clone()
		}
	}

	impl ToString for ReachabilityForSmsConfiguration {
		fn to_string(&self) -> String {
			match *self {
				Self::ReachabilityForSmsOverNas => "REACHABILITY_FOR_SMS_OVER_NAS".to_string(),
				Self::ReachabilityForSmsOverIp => "REACHABILITY_FOR_SMS_OVER_IP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReachabilityForSmsConfiguration {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REACHABILITY_FOR_SMS_OVER_NAS" => Ok(Self::ReachabilityForSmsOverNas),
				"REACHABILITY_FOR_SMS_OVER_IP" => Ok(Self::ReachabilityForSmsOverIp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReachabilityForSmsConfiguration {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReachabilityForSmsConfiguration {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReachabilityForSmsConfiguration {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ReachabilityForSmsReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "smsfAccessType"
	///  ],
	///  "properties": {
	///    "maxAvailabilityTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "smsfAccessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReachabilityForSmsReport {
		#[serde(
			rename = "maxAvailabilityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_availability_time: Option<DateTime>,
		#[serde(rename = "smsfAccessType")]
		pub smsf_access_type: AccessType,
	}

	impl From<&ReachabilityForSmsReport> for ReachabilityForSmsReport {
		fn from(value: &ReachabilityForSmsReport) -> Self {
			value.clone()
		}
	}

	/// ReachabilityReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "accessTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "idleStatusIndication": {
	///      "$ref": "#/components/schemas/IdleStatusIndication"
	///    },
	///    "maxAvailabilityTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "reachability": {
	///      "$ref": "#/components/schemas/UeReachability"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReachabilityReport {
		#[serde(
			rename = "accessTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_type_list: Vec<AccessType>,
		#[serde(
			rename = "amfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "idleStatusIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub idle_status_indication: Option<IdleStatusIndication>,
		#[serde(
			rename = "maxAvailabilityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_availability_time: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub reachability: Option<UeReachability>,
	}

	impl From<&ReachabilityReport> for ReachabilityReport {
		fn from(value: &ReachabilityReport) -> Self {
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

	/// RegistrationDataSetName
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "AMF_3GPP",
	///    "AMF_NON_3GPP",
	///    "SMF_PDU_SESSIONS",
	///    "SMSF_3GPP",
	///    "SMSF_NON_3GPP",
	///    "IP_SM_GW",
	///    "NWDAF"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RegistrationDataSetName {
		#[default]
		#[serde(rename = "AMF_3GPP")]
		Amf3gpp,
		#[serde(rename = "AMF_NON_3GPP")]
		AmfNon3gpp,
		#[serde(rename = "SMF_PDU_SESSIONS")]
		SmfPduSessions,
		#[serde(rename = "SMSF_3GPP")]
		Smsf3gpp,
		#[serde(rename = "SMSF_NON_3GPP")]
		SmsfNon3gpp,
		#[serde(rename = "IP_SM_GW")]
		IpSmGw,
		#[serde(rename = "NWDAF")]
		Nwdaf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RegistrationDataSetName> for RegistrationDataSetName {
		fn from(value: &RegistrationDataSetName) -> Self {
			value.clone()
		}
	}

	impl ToString for RegistrationDataSetName {
		fn to_string(&self) -> String {
			match *self {
				Self::Amf3gpp => "AMF_3GPP".to_string(),
				Self::AmfNon3gpp => "AMF_NON_3GPP".to_string(),
				Self::SmfPduSessions => "SMF_PDU_SESSIONS".to_string(),
				Self::Smsf3gpp => "SMSF_3GPP".to_string(),
				Self::SmsfNon3gpp => "SMSF_NON_3GPP".to_string(),
				Self::IpSmGw => "IP_SM_GW".to_string(),
				Self::Nwdaf => "NWDAF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RegistrationDataSetName {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AMF_3GPP" => Ok(Self::Amf3gpp),
				"AMF_NON_3GPP" => Ok(Self::AmfNon3gpp),
				"SMF_PDU_SESSIONS" => Ok(Self::SmfPduSessions),
				"SMSF_3GPP" => Ok(Self::Smsf3gpp),
				"SMSF_NON_3GPP" => Ok(Self::SmsfNon3gpp),
				"IP_SM_GW" => Ok(Self::IpSmGw),
				"NWDAF" => Ok(Self::Nwdaf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RegistrationDataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RegistrationDataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RegistrationDataSetName {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RegistrationDataSets
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "amf3Gpp": {
	///      "$ref": "#/components/schemas/Amf3GppAccessRegistration"
	///    },
	///    "amfNon3Gpp": {
	///      "$ref": "#/components/schemas/AmfNon3GppAccessRegistration"
	///    },
	///    "ipSmGw": {
	///      "$ref": "#/components/schemas/IpSmGwRegistration"
	///    },
	///    "nwdafRegistration": {
	///      "$ref": "#/components/schemas/NwdafRegistrationInfo"
	///    },
	///    "smfRegistration": {
	///      "$ref": "#/components/schemas/SmfRegistrationInfo"
	///    },
	///    "smsf3Gpp": {
	///      "$ref": "#/components/schemas/SmsfRegistration"
	///    },
	///    "smsfNon3Gpp": {
	///      "$ref": "#/components/schemas/SmsfRegistration"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RegistrationDataSets {
		#[serde(rename = "amf3Gpp", default, skip_serializing_if = "Option::is_none")]
		pub amf3_gpp: Option<Amf3GppAccessRegistration>,
		#[serde(
			rename = "amfNon3Gpp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_non3_gpp: Option<AmfNon3GppAccessRegistration>,
		#[serde(rename = "ipSmGw", default, skip_serializing_if = "Option::is_none")]
		pub ip_sm_gw: Option<IpSmGwRegistration>,
		#[serde(
			rename = "nwdafRegistration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_registration: Option<NwdafRegistrationInfo>,
		#[serde(
			rename = "smfRegistration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_registration: Option<SmfRegistrationInfo>,
		#[serde(rename = "smsf3Gpp", default, skip_serializing_if = "Option::is_none")]
		pub smsf3_gpp: Option<SmsfRegistration>,
		#[serde(
			rename = "smsfNon3Gpp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_non3_gpp: Option<SmsfRegistration>,
	}

	impl From<&RegistrationDataSets> for RegistrationDataSets {
		fn from(value: &RegistrationDataSets) -> Self {
			value.clone()
		}
	}

	/// RegistrationDatasetNames
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/RegistrationDataSetName"
	///  },
	///  "minItems": 2,
	///  "uniqueItems": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RegistrationDatasetNames(pub Vec<RegistrationDataSetName>);

	impl ::std::ops::Deref for RegistrationDatasetNames {
		type Target = Vec<RegistrationDataSetName>;
		fn deref(&self) -> &Vec<RegistrationDataSetName> {
			&self.0
		}
	}

	impl From<RegistrationDatasetNames> for Vec<RegistrationDataSetName> {
		fn from(value: RegistrationDatasetNames) -> Self {
			value.0
		}
	}

	impl From<&RegistrationDatasetNames> for RegistrationDatasetNames {
		fn from(value: &RegistrationDatasetNames) -> Self {
			value.clone()
		}
	}

	impl From<Vec<RegistrationDataSetName>> for RegistrationDatasetNames {
		fn from(value: Vec<RegistrationDataSetName>) -> Self {
			Self(value)
		}
	}

	/// RegistrationLocationInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "accessTypeList",
	///    "amfInstanceId"
	///  ],
	///  "properties": {
	///    "accessTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "amfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "vgmlcAddress": {
	///      "$ref": "#/components/schemas/VgmlcAddress"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RegistrationLocationInfo {
		#[serde(rename = "accessTypeList")]
		pub access_type_list: Vec<AccessType>,
		#[serde(rename = "amfInstanceId")]
		pub amf_instance_id: NfInstanceId,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<Guami>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(
			rename = "vgmlcAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vgmlc_address: Option<VgmlcAddress>,
	}

	impl From<&RegistrationLocationInfo> for RegistrationLocationInfo {
		fn from(value: &RegistrationLocationInfo) -> Self {
			value.clone()
		}
	}

	/// RegistrationReason
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SMF_CONTEXT_TRANSFERRED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RegistrationReason {
		#[default]
		#[serde(rename = "SMF_CONTEXT_TRANSFERRED")]
		SmfContextTransferred,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RegistrationReason> for RegistrationReason {
		fn from(value: &RegistrationReason) -> Self {
			value.clone()
		}
	}

	impl ToString for RegistrationReason {
		fn to_string(&self) -> String {
			match *self {
				Self::SmfContextTransferred => "SMF_CONTEXT_TRANSFERRED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RegistrationReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SMF_CONTEXT_TRANSFERRED" => Ok(Self::SmfContextTransferred),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RegistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RegistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RegistrationReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Report
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/ChangeOfSupiPeiAssociationReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/RoamingStatusReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/CnTypeChangeReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/CmInfoReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/LossConnectivityReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/LocationReport"
	///    },
	///    {
	///      "$ref": "#/components/schemas/PdnConnectivityStatReport"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum Report {
		#[default]
		ChangeOfSupiPeiAssociationReport(ChangeOfSupiPeiAssociationReport),
		RoamingStatusReport(RoamingStatusReport),
		CnTypeChangeReport(CnTypeChangeReport),
		CmInfoReport(CmInfoReport),
		LossConnectivityReport(LossConnectivityReport),
		LocationReport(LocationReport),
		PdnConnectivityStatReport(PdnConnectivityStatReport),
	}

	impl From<&Report> for Report {
		fn from(value: &Report) -> Self {
			value.clone()
		}
	}

	impl From<ChangeOfSupiPeiAssociationReport> for Report {
		fn from(value: ChangeOfSupiPeiAssociationReport) -> Self {
			Self::ChangeOfSupiPeiAssociationReport(value)
		}
	}

	impl From<RoamingStatusReport> for Report {
		fn from(value: RoamingStatusReport) -> Self {
			Self::RoamingStatusReport(value)
		}
	}

	impl From<CnTypeChangeReport> for Report {
		fn from(value: CnTypeChangeReport) -> Self {
			Self::CnTypeChangeReport(value)
		}
	}

	impl From<CmInfoReport> for Report {
		fn from(value: CmInfoReport) -> Self {
			Self::CmInfoReport(value)
		}
	}

	impl From<LossConnectivityReport> for Report {
		fn from(value: LossConnectivityReport) -> Self {
			Self::LossConnectivityReport(value)
		}
	}

	impl From<LocationReport> for Report {
		fn from(value: LocationReport) -> Self {
			Self::LocationReport(value)
		}
	}

	impl From<PdnConnectivityStatReport> for Report {
		fn from(value: PdnConnectivityStatReport) -> Self {
			Self::PdnConnectivityStatReport(value)
		}
	}

	/// ReportingOptions
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "guardTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "maxNumOfReports": {
	///      "$ref": "#/components/schemas/MaxNumOfReports"
	///    },
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "reportMode": {
	///      "$ref": "#/components/schemas/EventReportMode"
	///    },
	///    "reportPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "samplingRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingOptions {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(rename = "guardTime", default, skip_serializing_if = "Option::is_none")]
		pub guard_time: Option<DurationSec>,
		#[serde(
			rename = "maxNumOfReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_num_of_reports: Option<MaxNumOfReports>,
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		#[serde(
			rename = "reportMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_mode: Option<EventReportMode>,
		#[serde(
			rename = "reportPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_period: Option<DurationSec>,
		#[serde(
			rename = "samplingRatio",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sampling_ratio: Option<SamplingRatio>,
	}

	impl From<&ReportingOptions> for ReportingOptions {
		fn from(value: &ReportingOptions) -> Self {
			value.clone()
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

	/// RevokedCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NOT_ALLOWED",
	///    "EXCLUDED_FROM_GROUP",
	///    "GPSI_REMOVED"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RevokedCause {
		#[default]
		#[serde(rename = "NOT_ALLOWED")]
		NotAllowed,
		#[serde(rename = "EXCLUDED_FROM_GROUP")]
		ExcludedFromGroup,
		#[serde(rename = "GPSI_REMOVED")]
		GpsiRemoved,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RevokedCause> for RevokedCause {
		fn from(value: &RevokedCause) -> Self {
			value.clone()
		}
	}

	impl ToString for RevokedCause {
		fn to_string(&self) -> String {
			match *self {
				Self::NotAllowed => "NOT_ALLOWED".to_string(),
				Self::ExcludedFromGroup => "EXCLUDED_FROM_GROUP".to_string(),
				Self::GpsiRemoved => "GPSI_REMOVED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RevokedCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NOT_ALLOWED" => Ok(Self::NotAllowed),
				"EXCLUDED_FROM_GROUP" => Ok(Self::ExcludedFromGroup),
				"GPSI_REMOVED" => Ok(Self::GpsiRemoved),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RevokedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RevokedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RevokedCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RgAuthCtx
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "authInd"
	///  ],
	///  "properties": {
	///    "authInd": {
	///      "default": false,
	///      "type": "boolean"
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
	pub struct RgAuthCtx {
		#[serde(rename = "authInd")]
		pub auth_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&RgAuthCtx> for RgAuthCtx {
		fn from(value: &RgAuthCtx) -> Self {
			value.clone()
		}
	}

	/// Contains the Roaming Information Update
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Roaming Information Update",
	///  "type": "object",
	///  "required": [
	///    "servingPlmn"
	///  ],
	///  "properties": {
	///    "roaming": {
	///      "type": "boolean"
	///    },
	///    "servingPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoamingInfoUpdate {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub roaming: Option<bool>,
		#[serde(rename = "servingPlmn")]
		pub serving_plmn: PlmnId,
	}

	impl From<&RoamingInfoUpdate> for RoamingInfoUpdate {
		fn from(value: &RoamingInfoUpdate) -> Self {
			value.clone()
		}
	}

	/// RoamingStatusReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "newServingPlmn",
	///    "roaming"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "newServingPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "roaming": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoamingStatusReport {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(rename = "newServingPlmn")]
		pub new_serving_plmn: PlmnId,
		pub roaming: bool,
	}

	impl From<&RoamingStatusReport> for RoamingStatusReport {
		fn from(value: &RoamingStatusReport) -> Self {
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
		NewUnchecked
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

	/// Request body of the send-routing-info-sm custom operation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Request body of the send-routing-info-sm custom
	/// operation",
	///  "type": "object",
	///  "properties": {
	///    "correlationId": {
	///      "type": "string"
	///    },
	///    "ipSmGwInd": {
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
	pub struct RoutingInfoSmRequest {
		#[serde(
			rename = "correlationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub correlation_id: Option<String>,
		#[serde(rename = "ipSmGwInd", default)]
		pub ip_sm_gw_ind: bool,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&RoutingInfoSmRequest> for RoutingInfoSmRequest {
		fn from(value: &RoutingInfoSmRequest) -> Self {
			value.clone()
		}
	}

	/// Addressing information of available nodes for SMS delivery
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Addressing information of available nodes for SMS
	/// delivery",
	///  "type": "object",
	///  "properties": {
	///    "ipSmGw": {
	///      "$ref": "#/components/schemas/IpSmGwInfo"
	///    },
	///    "smsRouter": {
	///      "$ref": "#/components/schemas/SmsRouterInfo"
	///    },
	///    "smsf3Gpp": {
	///      "$ref": "#/components/schemas/SmsfRegistration"
	///    },
	///    "smsfNon3Gpp": {
	///      "$ref": "#/components/schemas/SmsfRegistration"
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
	pub struct RoutingInfoSmResponse {
		#[serde(rename = "ipSmGw", default, skip_serializing_if = "Option::is_none")]
		pub ip_sm_gw: Option<IpSmGwInfo>,
		#[serde(rename = "smsRouter", default, skip_serializing_if = "Option::is_none")]
		pub sms_router: Option<SmsRouterInfo>,
		#[serde(rename = "smsf3Gpp", default, skip_serializing_if = "Option::is_none")]
		pub smsf3_gpp: Option<SmsfRegistration>,
		#[serde(
			rename = "smsfNon3Gpp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_non3_gpp: Option<SmsfRegistration>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&RoutingInfoSmResponse> for RoutingInfoSmResponse {
		fn from(value: &RoutingInfoSmResponse) -> Self {
			value.clone()
		}
	}

	/// SchemasAppDescriptor
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "appId": {
	///      "type": "string"
	///    },
	///    "osId": {
	///      "$ref": "#/components/schemas/OsId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasAppDescriptor {
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<String>,
		#[serde(rename = "osId", default, skip_serializing_if = "Option::is_none")]
		pub os_id: Option<OsId>,
	}

	impl From<&SchemasAppDescriptor> for SchemasAppDescriptor {
		fn from(value: &SchemasAppDescriptor) -> Self {
			value.clone()
		}
	}

	/// Contains identities representing those UEs potentially affected by a
	/// data-loss event at the UDR
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains identities representing those UEs potentially
	/// affected by a data-loss event at the UDR",
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
	///    "lastReplicationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
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
	pub struct SchemasDataRestorationNotification {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "lastReplicationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_replication_time: Option<DateTime>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "sNssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssai_list: Vec<Snssai>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
	}

	impl From<&SchemasDataRestorationNotification> for SchemasDataRestorationNotification {
		fn from(value: &SchemasDataRestorationNotification) -> Self {
			value.clone()
		}
	}

	/// SchemasEpsInterworkingInfo
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
	///        "$ref": "#/components/schemas/schemas-EpsIwkPgw"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasEpsInterworkingInfo {
		/// A map (list of key-value pairs where Dnn serves as key) of
		/// EpsIwkPgws
		#[serde(
			rename = "epsIwkPgws",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub eps_iwk_pgws: ::std::collections::HashMap<String, SchemasEpsIwkPgw>,
	}

	impl From<&SchemasEpsInterworkingInfo> for SchemasEpsInterworkingInfo {
		fn from(value: &SchemasEpsInterworkingInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasEpsIwkPgw
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
	pub struct SchemasEpsIwkPgw {
		#[serde(rename = "pgwFqdn")]
		pub pgw_fqdn: Fqdn,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(rename = "smfInstanceId")]
		pub smf_instance_id: NfInstanceId,
	}

	impl From<&SchemasEpsIwkPgw> for SchemasEpsIwkPgw {
		fn from(value: &SchemasEpsIwkPgw) -> Self {
			value.clone()
		}
	}

	/// SchemasIpAddress
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
	pub enum SchemasIpAddress {
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

	impl From<&SchemasIpAddress> for SchemasIpAddress {
		fn from(value: &SchemasIpAddress) -> Self {
			value.clone()
		}
	}

	/// SchemasLpi
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "locationPrivacyInd"
	///  ],
	///  "properties": {
	///    "locationPrivacyInd": {
	///      "$ref": "#/components/schemas/LocationPrivacyInd"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/schemas-ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasLpi {
		#[serde(rename = "locationPrivacyInd")]
		pub location_privacy_ind: LocationPrivacyInd,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<SchemasValidTimePeriod>,
	}

	impl From<&SchemasLpi> for SchemasLpi {
		fn from(value: &SchemasLpi) -> Self {
			value.clone()
		}
	}

	/// Contains RAND and AUTS
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains RAND and AUTS",
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
	pub struct SchemasResynchronizationInfo {
		pub auts: Auts,
		pub rand: Rand,
	}

	impl From<&SchemasResynchronizationInfo> for SchemasResynchronizationInfo {
		fn from(value: &SchemasResynchronizationInfo) -> Self {
			value.clone()
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
	pub struct SchemasSecuredPacket(pub String);

	impl ::std::ops::Deref for SchemasSecuredPacket {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasSecuredPacket> for String {
		fn from(value: SchemasSecuredPacket) -> Self {
			value.0
		}
	}

	impl From<&SchemasSecuredPacket> for SchemasSecuredPacket {
		fn from(value: &SchemasSecuredPacket) -> Self {
			value.clone()
		}
	}

	impl From<String> for SchemasSecuredPacket {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasSecuredPacket {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SchemasSecuredPacket {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// SchemasSorInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "ackInd",
	///    "provisioningTime"
	///  ],
	///  "properties": {
	///    "ackInd": {
	///      "$ref": "#/components/schemas/AckInd"
	///    },
	///    "countersor": {
	///      "$ref": "#/components/schemas/CounterSor"
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sorCmci": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "sorMacIausf": {
	///      "$ref": "#/components/schemas/SorMac"
	///    },
	///    "sorTransparentContainer": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "steeringContainer": {
	///      "$ref": "#/components/schemas/schemas-SteeringContainer"
	///    },
	///    "storeSorCmciInMe": {
	///      "type": "boolean"
	///    },
	///    "usimSupportOfSorCmci": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSorInfo {
		#[serde(rename = "ackInd")]
		pub ack_ind: AckInd,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub countersor: Option<CounterSor>,
		#[serde(rename = "provisioningTime")]
		pub provisioning_time: DateTime,
		#[serde(rename = "sorCmci", default, skip_serializing_if = "Option::is_none")]
		pub sor_cmci: Option<Bytes>,
		#[serde(
			rename = "sorMacIausf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_mac_iausf: Option<SorMac>,
		#[serde(
			rename = "sorTransparentContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_transparent_container: Option<Bytes>,
		#[serde(
			rename = "steeringContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steering_container: Option<SchemasSteeringContainer>,
		#[serde(
			rename = "storeSorCmciInMe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub store_sor_cmci_in_me: Option<bool>,
		#[serde(
			rename = "usimSupportOfSorCmci",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub usim_support_of_sor_cmci: Option<bool>,
	}

	impl From<&SchemasSorInfo> for SchemasSorInfo {
		fn from(value: &SchemasSorInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasSteeringContainer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub enum SchemasSteeringContainer {
		#[default]
		Variant0(Vec<SteeringInfo>),
		Variant1(SecuredPacket),
	}

	impl From<&SchemasSteeringContainer> for SchemasSteeringContainer {
		fn from(value: &SchemasSteeringContainer) -> Self {
			value.clone()
		}
	}

	impl From<Vec<SteeringInfo>> for SchemasSteeringContainer {
		fn from(value: Vec<SteeringInfo>) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<SecuredPacket> for SchemasSteeringContainer {
		fn from(value: SecuredPacket) -> Self {
			Self::Variant1(value)
		}
	}

	/// SchemasValidTimePeriod
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "endTime": {
	///      "$ref": "#/components/schemas/DateTime"
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
	pub struct SchemasValidTimePeriod {
		#[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
		pub end_time: Option<DateTime>,
		#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
		pub start_time: Option<DateTime>,
	}

	impl From<&SchemasValidTimePeriod> for SchemasValidTimePeriod {
		fn from(value: &SchemasValidTimePeriod) -> Self {
			value.clone()
		}
	}

	/// SdmSubsModification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "expires": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "monitoredResourceUris": {
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
	pub struct SdmSubsModification {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expires: Option<DateTime>,
		#[serde(
			rename = "monitoredResourceUris",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub monitored_resource_uris: Vec<Uri>,
	}

	impl From<&SdmSubsModification> for SdmSubsModification {
		fn from(value: &SdmSubsModification) -> Self {
			value.clone()
		}
	}

	/// SdmSubscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "callbackReference",
	///    "monitoredResourceUris",
	///    "nfInstanceId"
	///  ],
	///  "properties": {
	///    "amfServiceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "callbackReference": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "expires": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "immediateReport": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "implicitUnsubscribe": {
	///      "type": "boolean"
	///    },
	///    "monitoredResourceUris": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "nfChangeFilter": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "report": {
	///      "$ref": "#/components/schemas/ImmediateReport"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "subscriptionId": {
	///      "type": "string"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueConSmfDataSubFilter": {
	///      "$ref": "#/components/schemas/UeContextInSmfDataSubFilter"
	///    },
	///    "uniqueSubscription": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SdmSubscription {
		#[serde(
			rename = "amfServiceName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_service_name: Option<ServiceName>,
		#[serde(rename = "callbackReference")]
		pub callback_reference: Uri,
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expires: Option<DateTime>,
		#[serde(rename = "immediateReport", default)]
		pub immediate_report: bool,
		#[serde(
			rename = "implicitUnsubscribe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub implicit_unsubscribe: Option<bool>,
		#[serde(rename = "monitoredResourceUris")]
		pub monitored_resource_uris: Vec<Uri>,
		#[serde(rename = "nfChangeFilter", default)]
		pub nf_change_filter: bool,
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub report: Option<ImmediateReport>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(
			rename = "singleNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub single_nssai: Option<Snssai>,
		#[serde(
			rename = "subscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscription_id: Option<String>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
		#[serde(
			rename = "ueConSmfDataSubFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_con_smf_data_sub_filter: Option<UeContextInSmfDataSubFilter>,
		#[serde(
			rename = "uniqueSubscription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unique_subscription: Option<bool>,
	}

	impl From<&SdmSubscription> for SdmSubscription {
		fn from(value: &SdmSubscription) -> Self {
			value.clone()
		}
	}

	/// SecuredPacket
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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

	/// Authorization Response for a specific service.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Authorization Response for a specific service.",
	///  "type": "object",
	///  "properties": {
	///    "authId": {
	///      "type": "string"
	///    },
	///    "authorizationUeId": {
	///      "$ref": "#/components/schemas/AuthorizationUeId"
	///    },
	///    "extGroupId": {
	///      "$ref": "#/components/schemas/ExternalGroupId"
	///    },
	///    "intGroupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceSpecificAuthorizationData {
		#[serde(rename = "authId", default, skip_serializing_if = "Option::is_none")]
		pub auth_id: Option<String>,
		#[serde(
			rename = "authorizationUeId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub authorization_ue_id: Option<AuthorizationUeId>,
		#[serde(
			rename = "extGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_group_id: Option<ExternalGroupId>,
		#[serde(
			rename = "intGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub int_group_id: Option<GroupId>,
	}

	impl From<&ServiceSpecificAuthorizationData> for ServiceSpecificAuthorizationData {
		fn from(value: &ServiceSpecificAuthorizationData) -> Self {
			value.clone()
		}
	}

	/// Authorization information for a specific service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Authorization information for a specific service",
	///  "type": "object",
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "authUpdateCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "nefId": {
	///      "$ref": "#/components/schemas/NefId"
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
	pub struct ServiceSpecificAuthorizationInfo {
		#[serde(rename = "afId", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<String>,
		#[serde(
			rename = "authUpdateCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_update_callback_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(rename = "nefId", default, skip_serializing_if = "Option::is_none")]
		pub nef_id: Option<NefId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&ServiceSpecificAuthorizationInfo> for ServiceSpecificAuthorizationInfo {
		fn from(value: &ServiceSpecificAuthorizationInfo) -> Self {
			value.clone()
		}
	}

	/// Information for Authorization removal of a specific service.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information for Authorization removal of a specific
	/// service.",
	///  "type": "object",
	///  "required": [
	///    "authId"
	///  ],
	///  "properties": {
	///    "authId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceSpecificAuthorizationRemoveData {
		#[serde(rename = "authId")]
		pub auth_id: String,
	}

	impl From<&ServiceSpecificAuthorizationRemoveData> for ServiceSpecificAuthorizationRemoveData {
		fn from(value: &ServiceSpecificAuthorizationRemoveData) -> Self {
			value.clone()
		}
	}

	/// Possible values are - AF_GUIDANCE_FOR_URSP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are - AF_GUIDANCE_FOR_URSP\n",
	///  "type": "string",
	///  "enum": [
	///    "AF_GUIDANCE_FOR_URSP"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum ServiceType {
		#[default]
		#[serde(rename = "AF_GUIDANCE_FOR_URSP")]
		AfGuidanceForUrsp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ServiceType> for ServiceType {
		fn from(value: &ServiceType) -> Self {
			value.clone()
		}
	}

	impl ToString for ServiceType {
		fn to_string(&self) -> String {
			match *self {
				Self::AfGuidanceForUrsp => "AF_GUIDANCE_FOR_URSP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ServiceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AF_GUIDANCE_FOR_URSP" => Ok(Self::AfGuidanceForUrsp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ServiceTypeUnrelatedClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "serviceType"
	///  ],
	///  "properties": {
	///    "allowedGeographicArea": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 1
	///    },
	///    "codeWordInd": {
	///      "$ref": "#/components/schemas/CodeWordInd"
	///    },
	///    "codeWordList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodeWord"
	///      },
	///      "minItems": 1
	///    },
	///    "privacyCheckRelatedAction": {
	///      "$ref": "#/components/schemas/PrivacyCheckRelatedAction"
	///    },
	///    "serviceType": {
	///      "$ref": "#/components/schemas/LcsServiceType"
	///    },
	///    "validTimePeriod": {
	///      "$ref": "#/components/schemas/ValidTimePeriod"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceTypeUnrelatedClass {
		#[serde(
			rename = "allowedGeographicArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_geographic_area: Vec<GeographicArea>,
		#[serde(
			rename = "codeWordInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub code_word_ind: Option<CodeWordInd>,
		#[serde(
			rename = "codeWordList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub code_word_list: Vec<CodeWord>,
		#[serde(
			rename = "privacyCheckRelatedAction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub privacy_check_related_action: Option<PrivacyCheckRelatedAction>,
		#[serde(rename = "serviceType")]
		pub service_type: LcsServiceType,
		#[serde(
			rename = "validTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub valid_time_period: Option<ValidTimePeriod>,
	}

	impl From<&ServiceTypeUnrelatedClass> for ServiceTypeUnrelatedClass {
		fn from(value: &ServiceTypeUnrelatedClass) -> Self {
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
		NewUnchecked
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

	/// SessionManagementSubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "singleNssai"
	///  ],
	///  "properties": {
	///    "3gppChargingCharacteristics": {
	///      "$ref": "#/components/schemas/3GppChargingCharacteristics"
	///    },
	///    "dnnConfigurations": {
	///      "description": "A map (list of key-value pairs where Dnn, or
	/// optionally the Wildcard DNN, serves as key) of DnnConfigurations",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DnnConfiguration"
	///      }
	///    },
	///    "expectedUeBehavioursList": {
	///      "description": "A map(list of key-value pairs) where Dnn serves as
	/// key of ExpectedUeBehaviourData",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/ExpectedUeBehaviourData"
	///      }
	///    },
	///    "internalGroupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "odbPacketServices": {
	///      "$ref": "#/components/schemas/OdbPacketServices"
	///    },
	///    "sharedDnnConfigurationsId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "sharedTraceDataId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "sharedVnGroupDataIds": {
	///      "description": "A map(list of key-value pairs) where GroupId serves
	/// as key of SharedDataId",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SharedDataId"
	///      }
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "suggestedPacketNumDlList": {
	///      "description": "A map(list of key-value pairs) where Dnn serves as
	/// key of SuggestedPacketNumDl",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SuggestedPacketNumDl"
	///      }
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SessionManagementSubscriptionData {
		/// A map (list of key-value pairs where Dnn, or optionally the Wildcard
		/// DNN, serves as key) of DnnConfigurations
		#[serde(
			rename = "dnnConfigurations",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub dnn_configurations: ::std::collections::HashMap<String, DnnConfiguration>,
		/// A map(list of key-value pairs) where Dnn serves as key of
		/// ExpectedUeBehaviourData
		#[serde(
			rename = "expectedUeBehavioursList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub expected_ue_behaviours_list:
			::std::collections::HashMap<String, ExpectedUeBehaviourData>,
		#[serde(
			rename = "internalGroupIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_ids: Vec<GroupId>,
		#[serde(
			rename = "odbPacketServices",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub odb_packet_services: Option<OdbPacketServices>,
		#[serde(
			rename = "sharedDnnConfigurationsId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_dnn_configurations_id: Option<SharedDataId>,
		#[serde(
			rename = "sharedTraceDataId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_trace_data_id: Option<SharedDataId>,
		/// A map(list of key-value pairs) where GroupId serves as key of
		/// SharedDataId
		#[serde(
			rename = "sharedVnGroupDataIds",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub shared_vn_group_data_ids: ::std::collections::HashMap<String, SharedDataId>,
		#[serde(rename = "singleNssai")]
		pub single_nssai: Snssai,
		/// A map(list of key-value pairs) where Dnn serves as key of
		/// SuggestedPacketNumDl
		#[serde(
			rename = "suggestedPacketNumDlList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub suggested_packet_num_dl_list: ::std::collections::HashMap<String, SuggestedPacketNumDl>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "3gppChargingCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_charging_characteristics: Option<_3gppChargingCharacteristics>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
	}

	impl From<&SessionManagementSubscriptionData> for SessionManagementSubscriptionData {
		fn from(value: &SessionManagementSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// SharedData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "sharedDataId"
	///  ],
	///  "properties": {
	///    "sharedAmData": {
	///      "$ref": "#/components/schemas/AccessAndMobilitySubscriptionData"
	///    },
	///    "sharedDataId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "sharedDnnConfigurations": {
	///      "description": "A map(list of key-value pairs) where Dnn, or
	/// optionally the Wildcard DNN, serves as key of DnnConfiguration",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DnnConfiguration"
	///      }
	///    },
	///    "sharedEcsAddrConfigInfo": {
	///      "$ref": "#/components/schemas/EcsAddrConfigInfo"
	///    },
	///    "sharedSmSubsData": {
	///      "$ref": "#/components/schemas/SessionManagementSubscriptionData"
	///    },
	///    "sharedSmsMngSubsData": {
	///      "$ref": "#/components/schemas/SmsManagementSubscriptionData"
	///    },
	///    "sharedSmsSubsData": {
	///      "$ref": "#/components/schemas/SmsSubscriptionData"
	///    },
	///    "sharedSnssaiInfos": {
	///      "description": "A map(list of key-value pairs) where singleNssai
	/// serves as key of SnssaiInfo",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SnssaiInfo"
	///      }
	///    },
	///    "sharedTraceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "sharedVnGroupDatas": {
	///      "description": "A map(list of key-value pairs) where GroupId serves
	/// as key of VnGroupData",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/VnGroupData"
	///      }
	///    },
	///    "treatmentInstructions": {
	///      "description": "A map(list of key-value pairs) where JSON pointer
	/// pointing to an attribute within the SharedData serves as key of
	/// SharedDataTreatmentInstruction",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SharedDataTreatmentInstruction"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SharedData {
		#[serde(
			rename = "sharedAmData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_am_data: Option<AccessAndMobilitySubscriptionData>,
		#[serde(rename = "sharedDataId")]
		pub shared_data_id: SharedDataId,
		/// A map(list of key-value pairs) where Dnn, or optionally the Wildcard
		/// DNN, serves as key of DnnConfiguration
		#[serde(
			rename = "sharedDnnConfigurations",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub shared_dnn_configurations: ::std::collections::HashMap<String, DnnConfiguration>,
		#[serde(
			rename = "sharedEcsAddrConfigInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_ecs_addr_config_info: Option<EcsAddrConfigInfo>,
		#[serde(
			rename = "sharedSmSubsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_sm_subs_data: Option<SessionManagementSubscriptionData>,
		#[serde(
			rename = "sharedSmsMngSubsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_sms_mng_subs_data: Option<SmsManagementSubscriptionData>,
		#[serde(
			rename = "sharedSmsSubsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_sms_subs_data: Option<SmsSubscriptionData>,
		/// A map(list of key-value pairs) where singleNssai serves as key of
		/// SnssaiInfo
		#[serde(
			rename = "sharedSnssaiInfos",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub shared_snssai_infos: ::std::collections::HashMap<String, SnssaiInfo>,
		#[serde(
			rename = "sharedTraceData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_trace_data: Option<TraceData>,
		/// A map(list of key-value pairs) where GroupId serves as key of
		/// VnGroupData
		#[serde(
			rename = "sharedVnGroupDatas",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub shared_vn_group_datas: ::std::collections::HashMap<String, VnGroupData>,
		/// A map(list of key-value pairs) where JSON pointer pointing to an
		/// attribute within the SharedData serves as key of
		/// SharedDataTreatmentInstruction
		#[serde(
			rename = "treatmentInstructions",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub treatment_instructions:
			::std::collections::HashMap<String, SharedDataTreatmentInstruction>,
	}

	impl From<&SharedData> for SharedData {
		fn from(value: &SharedData) -> Self {
			value.clone()
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
		NewUnchecked
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

	/// SharedDataIds
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/SharedDataId"
	///  },
	///  "minItems": 1,
	///  "uniqueItems": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SharedDataIds(pub Vec<SharedDataId>);

	impl ::std::ops::Deref for SharedDataIds {
		type Target = Vec<SharedDataId>;
		fn deref(&self) -> &Vec<SharedDataId> {
			&self.0
		}
	}

	impl From<SharedDataIds> for Vec<SharedDataId> {
		fn from(value: SharedDataIds) -> Self {
			value.0
		}
	}

	impl From<&SharedDataIds> for SharedDataIds {
		fn from(value: &SharedDataIds) -> Self {
			value.clone()
		}
	}

	impl From<Vec<SharedDataId>> for SharedDataIds {
		fn from(value: Vec<SharedDataId>) -> Self {
			Self(value)
		}
	}

	/// SharedDataTreatmentInstruction
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "USE_IF_NO_CLASH",
	///    "OVERWRITE",
	///    "MAX",
	///    "MIN"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SharedDataTreatmentInstruction {
		#[default]
		#[serde(rename = "USE_IF_NO_CLASH")]
		UseIfNoClash,
		#[serde(rename = "OVERWRITE")]
		Overwrite,
		#[serde(rename = "MAX")]
		Max,
		#[serde(rename = "MIN")]
		Min,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SharedDataTreatmentInstruction> for SharedDataTreatmentInstruction {
		fn from(value: &SharedDataTreatmentInstruction) -> Self {
			value.clone()
		}
	}

	impl ToString for SharedDataTreatmentInstruction {
		fn to_string(&self) -> String {
			match *self {
				Self::UseIfNoClash => "USE_IF_NO_CLASH".to_string(),
				Self::Overwrite => "OVERWRITE".to_string(),
				Self::Max => "MAX".to_string(),
				Self::Min => "MIN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SharedDataTreatmentInstruction {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"USE_IF_NO_CLASH" => Ok(Self::UseIfNoClash),
				"OVERWRITE" => Ok(Self::Overwrite),
				"MAX" => Ok(Self::Max),
				"MIN" => Ok(Self::Min),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SharedDataTreatmentInstruction {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SharedDataTreatmentInstruction {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SharedDataTreatmentInstruction {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents SM Delivery Status.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents SM Delivery Status.",
	///  "type": "object",
	///  "required": [
	///    "gpsi",
	///    "smStatusReport"
	///  ],
	///  "properties": {
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "smStatusReport": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmDeliveryStatus {
		pub gpsi: Gpsi,
		#[serde(rename = "smStatusReport")]
		pub sm_status_report: String,
	}

	impl From<&SmDeliveryStatus> for SmDeliveryStatus {
		fn from(value: &SmDeliveryStatus) -> Self {
			value.clone()
		}
	}

	/// SmSubsData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SessionManagementSubscriptionData"
	///      },
	///      "minItems": 1
	///    },
	///    {
	///      "$ref": "#/components/schemas/ExtendedSmSubsData"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum SmSubsData {
		#[default]
		Variant0(Vec<SessionManagementSubscriptionData>),
		Variant1(ExtendedSmSubsData),
	}

	impl From<&SmSubsData> for SmSubsData {
		fn from(value: &SmSubsData) -> Self {
			value.clone()
		}
	}

	impl From<Vec<SessionManagementSubscriptionData>> for SmSubsData {
		fn from(value: Vec<SessionManagementSubscriptionData>) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<ExtendedSmSubsData> for SmSubsData {
		fn from(value: ExtendedSmSubsData) -> Self {
			Self::Variant1(value)
		}
	}

	/// SmfRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "pduSessionId",
	///    "plmnId",
	///    "singleNssai",
	///    "smfInstanceId"
	///  ],
	///  "properties": {
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "deregCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "emergencyServices": {
	///      "type": "boolean"
	///    },
	///    "epdgInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "lastSynchronizationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "pcscfRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
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
	///    "registrationReason": {
	///      "$ref": "#/components/schemas/RegistrationReason"
	///    },
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "smfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
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
	pub struct SmfRegistration {
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(
			rename = "deregCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dereg_callback_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "emergencyServices",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub emergency_services: Option<bool>,
		#[serde(rename = "epdgInd", default)]
		pub epdg_ind: bool,
		#[serde(
			rename = "lastSynchronizationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_synchronization_time: Option<DateTime>,
		#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_id: Option<NfInstanceId>,
		#[serde(
			rename = "pcscfRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcscf_restoration_callback_uri: Option<Uri>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(rename = "pgwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pgw_fqdn: Option<Fqdn>,
		#[serde(rename = "pgwIpAddr", default, skip_serializing_if = "Option::is_none")]
		pub pgw_ip_addr: Option<IpAddress>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(
			rename = "registrationReason",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_reason: Option<RegistrationReason>,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "singleNssai")]
		pub single_nssai: Snssai,
		#[serde(rename = "smfInstanceId")]
		pub smf_instance_id: NfInstanceId,
		#[serde(rename = "smfSetId", default, skip_serializing_if = "Option::is_none")]
		pub smf_set_id: Option<NfSetId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
	}

	impl From<&SmfRegistration> for SmfRegistration {
		fn from(value: &SmfRegistration) -> Self {
			value.clone()
		}
	}

	/// SmfRegistrationInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "smfRegistrationList"
	///  ],
	///  "properties": {
	///    "smfRegistrationList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SmfRegistration"
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
	pub struct SmfRegistrationInfo {
		#[serde(rename = "smfRegistrationList")]
		pub smf_registration_list: Vec<SmfRegistration>,
	}

	impl From<&SmfRegistrationInfo> for SmfRegistrationInfo {
		fn from(value: &SmfRegistrationInfo) -> Self {
			value.clone()
		}
	}

	/// Contains attributes of SmfRegistration that can be modified using PATCH
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains attributes of SmfRegistration that can be
	/// modified using PATCH",
	///  "type": "object",
	///  "required": [
	///    "smfInstanceId"
	///  ],
	///  "properties": {
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/FqdnRm"
	///    },
	///    "smfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmfRegistrationModification {
		#[serde(rename = "pgwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pgw_fqdn: Option<FqdnRm>,
		#[serde(rename = "smfInstanceId")]
		pub smf_instance_id: NfInstanceId,
		#[serde(rename = "smfSetId", default, skip_serializing_if = "Option::is_none")]
		pub smf_set_id: Option<NfSetId>,
	}

	impl From<&SmfRegistrationModification> for SmfRegistrationModification {
		fn from(value: &SmfRegistrationModification) -> Self {
			value.clone()
		}
	}

	/// SmfSelectionSubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "hssGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "sharedSnssaiInfosId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "subscribedSnssaiInfos": {
	///      "description": "A map(list of key-value pairs) where singleNssai
	/// serves as key of SnssaiInfo",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SnssaiInfo"
	///      }
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
	pub struct SmfSelectionSubscriptionData {
		#[serde(
			rename = "hssGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hss_group_id: Option<NfGroupId>,
		#[serde(
			rename = "sharedSnssaiInfosId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_snssai_infos_id: Option<SharedDataId>,
		/// A map(list of key-value pairs) where singleNssai serves as key of
		/// SnssaiInfo
		#[serde(
			rename = "subscribedSnssaiInfos",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub subscribed_snssai_infos: ::std::collections::HashMap<String, SnssaiInfo>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&SmfSelectionSubscriptionData> for SmfSelectionSubscriptionData {
		fn from(value: &SmfSelectionSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// SmsManagementSubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "moSmsBarringAll": {
	///      "type": "boolean"
	///    },
	///    "moSmsBarringRoaming": {
	///      "type": "boolean"
	///    },
	///    "moSmsSubscribed": {
	///      "type": "boolean"
	///    },
	///    "mtSmsBarringAll": {
	///      "type": "boolean"
	///    },
	///    "mtSmsBarringRoaming": {
	///      "type": "boolean"
	///    },
	///    "mtSmsSubscribed": {
	///      "type": "boolean"
	///    },
	///    "sharedSmsMngDataIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataId"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmsManagementSubscriptionData {
		#[serde(
			rename = "moSmsBarringAll",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_sms_barring_all: Option<bool>,
		#[serde(
			rename = "moSmsBarringRoaming",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_sms_barring_roaming: Option<bool>,
		#[serde(
			rename = "moSmsSubscribed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_sms_subscribed: Option<bool>,
		#[serde(
			rename = "mtSmsBarringAll",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mt_sms_barring_all: Option<bool>,
		#[serde(
			rename = "mtSmsBarringRoaming",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mt_sms_barring_roaming: Option<bool>,
		#[serde(
			rename = "mtSmsSubscribed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mt_sms_subscribed: Option<bool>,
		#[serde(
			rename = "sharedSmsMngDataIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub shared_sms_mng_data_ids: Vec<SharedDataId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
	}

	impl From<&SmsManagementSubscriptionData> for SmsManagementSubscriptionData {
		fn from(value: &SmsManagementSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// Addressing information of the SMS Router configured at the UDM
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Addressing information of the SMS Router configured at
	/// the UDM",
	///  "type": "object",
	///  "properties": {
	///    "diameterAddress": {
	///      "$ref": "#/components/schemas/NetworkNodeDiameterAddress"
	///    },
	///    "mapAddress": {
	///      "$ref": "#/components/schemas/E164Number"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "routerFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "routerIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "routerIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmsRouterInfo {
		#[serde(
			rename = "diameterAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub diameter_address: Option<NetworkNodeDiameterAddress>,
		#[serde(
			rename = "mapAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub map_address: Option<E164Number>,
		#[serde(
			rename = "nfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "routerFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub router_fqdn: Option<Fqdn>,
		#[serde(
			rename = "routerIpv4",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub router_ipv4: Option<Ipv4Addr>,
		#[serde(
			rename = "routerIpv6",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub router_ipv6: Option<Ipv6Addr>,
	}

	impl From<&SmsRouterInfo> for SmsRouterInfo {
		fn from(value: &SmsRouterInfo) -> Self {
			value.clone()
		}
	}

	/// SmsSubscribed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmsSubscribed(pub bool);

	impl ::std::ops::Deref for SmsSubscribed {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<SmsSubscribed> for bool {
		fn from(value: SmsSubscribed) -> Self {
			value.0
		}
	}

	impl From<&SmsSubscribed> for SmsSubscribed {
		fn from(value: &SmsSubscribed) -> Self {
			value.clone()
		}
	}

	impl From<bool> for SmsSubscribed {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SmsSubscribed {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SmsSubscribed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmsSubscribed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmsSubscribed {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SmsSubscribed {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// SmsSubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "sharedSmsSubsDataId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "smsSubscribed": {
	///      "$ref": "#/components/schemas/SmsSubscribed"
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
	pub struct SmsSubscriptionData {
		#[serde(
			rename = "sharedSmsSubsDataId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_sms_subs_data_id: Option<SharedDataId>,
		#[serde(
			rename = "smsSubscribed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sms_subscribed: Option<SmsSubscribed>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&SmsSubscriptionData> for SmsSubscriptionData {
		fn from(value: &SmsSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// SmsfInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "smsfInstanceId"
	///  ],
	///  "properties": {
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "smsfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smsfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmsfInfo {
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "smsfInstanceId")]
		pub smsf_instance_id: NfInstanceId,
		#[serde(rename = "smsfSetId", default, skip_serializing_if = "Option::is_none")]
		pub smsf_set_id: Option<NfSetId>,
	}

	impl From<&SmsfInfo> for SmsfInfo {
		fn from(value: &SmsfInfo) -> Self {
			value.clone()
		}
	}

	/// SmsfRegistration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "smsfInstanceId"
	///  ],
	///  "properties": {
	///    "contextInfo": {
	///      "$ref": "#/components/schemas/ContextInfo"
	///    },
	///    "dataRestorationCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "lastSynchronizationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "registrationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "smsfDiameterAddress": {
	///      "$ref": "#/components/schemas/NetworkNodeDiameterAddress"
	///    },
	///    "smsfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smsfMAPAddress": {
	///      "$ref": "#/components/schemas/E164Number"
	///    },
	///    "smsfSbiSupInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smsfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udrRestartInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueMemoryAvailableInd": {
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
	pub struct SmsfRegistration {
		#[serde(
			rename = "contextInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub context_info: Option<ContextInfo>,
		#[serde(
			rename = "dataRestorationCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_restoration_callback_uri: Option<Uri>,
		#[serde(
			rename = "lastSynchronizationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_synchronization_time: Option<DateTime>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(
			rename = "registrationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_time: Option<DateTime>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(
			rename = "smsfDiameterAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_diameter_address: Option<NetworkNodeDiameterAddress>,
		#[serde(rename = "smsfInstanceId")]
		pub smsf_instance_id: NfInstanceId,
		#[serde(
			rename = "smsfMAPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_map_address: Option<E164Number>,
		#[serde(rename = "smsfSbiSupInd", default)]
		pub smsf_sbi_sup_ind: bool,
		#[serde(rename = "smsfSetId", default, skip_serializing_if = "Option::is_none")]
		pub smsf_set_id: Option<NfSetId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "udrRestartInd", default)]
		pub udr_restart_ind: bool,
		#[serde(
			rename = "ueMemoryAvailableInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_memory_available_ind: Option<bool>,
	}

	impl From<&SmsfRegistration> for SmsfRegistration {
		fn from(value: &SmsfRegistration) -> Self {
			value.clone()
		}
	}

	/// SnssaiInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnnInfos"
	///  ],
	///  "properties": {
	///    "dnnInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnInfo"
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
	pub struct SnssaiInfo {
		#[serde(rename = "dnnInfos")]
		pub dnn_infos: Vec<DnnInfo>,
	}

	impl From<&SnssaiInfo> for SnssaiInfo {
		fn from(value: &SnssaiInfo) -> Self {
			value.clone()
		}
	}

	/// SorCmci
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
	pub struct SorCmci(pub Bytes);

	impl ::std::ops::Deref for SorCmci {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<SorCmci> for Bytes {
		fn from(value: SorCmci) -> Self {
			value.0
		}
	}

	impl From<&SorCmci> for SorCmci {
		fn from(value: &SorCmci) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for SorCmci {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SorCmci {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SorCmci {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SorCmci {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SorCmci {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SorCmci {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// SorInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "ackInd",
	///    "provisioningTime"
	///  ],
	///  "properties": {
	///    "ackInd": {
	///      "$ref": "#/components/schemas/AckInd"
	///    },
	///    "countersor": {
	///      "$ref": "#/components/schemas/CounterSor"
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sorCmci": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "sorMacIausf": {
	///      "$ref": "#/components/schemas/SorMac"
	///    },
	///    "sorTransparentContainer": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "steeringContainer": {
	///      "$ref": "#/components/schemas/SteeringContainer"
	///    },
	///    "storeSorCmciInMe": {
	///      "type": "boolean"
	///    },
	///    "usimSupportOfSorCmci": {
	///      "type": "boolean"
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
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub countersor: Option<CounterSor>,
		#[serde(rename = "provisioningTime")]
		pub provisioning_time: DateTime,
		#[serde(rename = "sorCmci", default, skip_serializing_if = "Option::is_none")]
		pub sor_cmci: Option<Bytes>,
		#[serde(
			rename = "sorMacIausf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_mac_iausf: Option<SorMac>,
		#[serde(
			rename = "sorTransparentContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_transparent_container: Option<Bytes>,
		#[serde(
			rename = "steeringContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steering_container: Option<SteeringContainer>,
		#[serde(
			rename = "storeSorCmciInMe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub store_sor_cmci_in_me: Option<bool>,
		#[serde(
			rename = "usimSupportOfSorCmci",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub usim_support_of_sor_cmci: Option<bool>,
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
		NewUnchecked
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

	/// SorTransparentContainer
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
	pub struct SorTransparentContainer(pub Bytes);

	impl ::std::ops::Deref for SorTransparentContainer {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<SorTransparentContainer> for Bytes {
		fn from(value: SorTransparentContainer) -> Self {
			value.0
		}
	}

	impl From<&SorTransparentContainer> for SorTransparentContainer {
		fn from(value: &SorTransparentContainer) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for SorTransparentContainer {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SorTransparentContainer {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SorTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SorTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SorTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SorTransparentContainer {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// SorUpdateIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INITIAL_REGISTRATION",
	///    "EMERGENCY_REGISTRATION"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum SorUpdateIndicator {
		#[default]
		#[serde(rename = "INITIAL_REGISTRATION")]
		InitialRegistration,
		#[serde(rename = "EMERGENCY_REGISTRATION")]
		EmergencyRegistration,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SorUpdateIndicator> for SorUpdateIndicator {
		fn from(value: &SorUpdateIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for SorUpdateIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::InitialRegistration => "INITIAL_REGISTRATION".to_string(),
				Self::EmergencyRegistration => "EMERGENCY_REGISTRATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SorUpdateIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INITIAL_REGISTRATION" => Ok(Self::InitialRegistration),
				"EMERGENCY_REGISTRATION" => Ok(Self::EmergencyRegistration),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SorUpdateIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SorUpdateIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SorUpdateIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SorUpdateInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "vplmnId"
	///  ],
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "vplmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SorUpdateInfo {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "vplmnId")]
		pub vplmn_id: PlmnId,
	}

	impl From<&SorUpdateInfo> for SorUpdateInfo {
		fn from(value: &SorUpdateInfo) -> Self {
			value.clone()
		}
	}

	/// SscModes
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "defaultSscMode"
	///  ],
	///  "properties": {
	///    "allowedSscModes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SscMode"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "defaultSscMode": {
	///      "$ref": "#/components/schemas/SscMode"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SscModes {
		#[serde(
			rename = "allowedSscModes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_ssc_modes: Vec<SscMode>,
		#[serde(rename = "defaultSscMode")]
		pub default_ssc_mode: SscMode,
	}

	impl From<&SscModes> for SscModes {
		fn from(value: &SscModes) -> Self {
			value.clone()
		}
	}

	/// SteeringContainer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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

	/// SubscriptionDataSets
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "amData": {
	///      "$ref": "#/components/schemas/AccessAndMobilitySubscriptionData"
	///    },
	///    "lcsBroadcastAssistanceTypesData": {
	///      "$ref": "#/components/schemas/LcsBroadcastAssistanceTypesData"
	///    },
	///    "lcsMoData": {
	///      "$ref": "#/components/schemas/LcsMoData"
	///    },
	///    "lcsPrivacyData": {
	///      "$ref": "#/components/schemas/LcsPrivacyData"
	///    },
	///    "mbsData": {
	///      "$ref": "#/components/schemas/MbsSubscriptionData"
	///    },
	///    "proseData": {
	///      "$ref": "#/components/schemas/ProseSubscriptionData"
	///    },
	///    "smData": {
	///      "$ref": "#/components/schemas/SmSubsData"
	///    },
	///    "smfSelData": {
	///      "$ref": "#/components/schemas/SmfSelectionSubscriptionData"
	///    },
	///    "smsMngData": {
	///      "$ref": "#/components/schemas/SmsManagementSubscriptionData"
	///    },
	///    "smsSubsData": {
	///      "$ref": "#/components/schemas/SmsSubscriptionData"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "ucData": {
	///      "$ref": "#/components/schemas/UcSubscriptionData"
	///    },
	///    "uecAmfData": {
	///      "$ref": "#/components/schemas/UeContextInAmfData"
	///    },
	///    "uecSmfData": {
	///      "$ref": "#/components/schemas/UeContextInSmfData"
	///    },
	///    "uecSmsfData": {
	///      "$ref": "#/components/schemas/UeContextInSmsfData"
	///    },
	///    "v2xData": {
	///      "$ref": "#/components/schemas/V2xSubscriptionData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SubscriptionDataSets {
		#[serde(rename = "amData", default, skip_serializing_if = "Option::is_none")]
		pub am_data: Option<AccessAndMobilitySubscriptionData>,
		#[serde(
			rename = "lcsBroadcastAssistanceTypesData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_broadcast_assistance_types_data: Option<LcsBroadcastAssistanceTypesData>,
		#[serde(rename = "lcsMoData", default, skip_serializing_if = "Option::is_none")]
		pub lcs_mo_data: Option<LcsMoData>,
		#[serde(
			rename = "lcsPrivacyData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_privacy_data: Option<LcsPrivacyData>,
		#[serde(rename = "mbsData", default, skip_serializing_if = "Option::is_none")]
		pub mbs_data: Option<MbsSubscriptionData>,
		#[serde(rename = "proseData", default, skip_serializing_if = "Option::is_none")]
		pub prose_data: Option<ProseSubscriptionData>,
		#[serde(rename = "smData", default, skip_serializing_if = "Option::is_none")]
		pub sm_data: Option<SmSubsData>,
		#[serde(
			rename = "smfSelData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_sel_data: Option<SmfSelectionSubscriptionData>,
		#[serde(
			rename = "smsMngData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sms_mng_data: Option<SmsManagementSubscriptionData>,
		#[serde(
			rename = "smsSubsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sms_subs_data: Option<SmsSubscriptionData>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(rename = "ucData", default, skip_serializing_if = "Option::is_none")]
		pub uc_data: Option<UcSubscriptionData>,
		#[serde(
			rename = "uecAmfData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uec_amf_data: Option<UeContextInAmfData>,
		#[serde(
			rename = "uecSmfData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uec_smf_data: Option<UeContextInSmfData>,
		#[serde(
			rename = "uecSmsfData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uec_smsf_data: Option<UeContextInSmsfData>,
		#[serde(rename = "v2xData", default, skip_serializing_if = "Option::is_none")]
		pub v2x_data: Option<V2xSubscriptionData>,
	}

	impl From<&SubscriptionDataSets> for SubscriptionDataSets {
		fn from(value: &SubscriptionDataSets) -> Self {
			value.clone()
		}
	}

	/// Success
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Success(pub bool);

	impl ::std::ops::Deref for Success {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<Success> for bool {
		fn from(value: Success) -> Self {
			value.0
		}
	}

	impl From<&Success> for Success {
		fn from(value: &Success) -> Self {
			value.clone()
		}
	}

	impl From<bool> for Success {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Success {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Success {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Success {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Success {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Success {
		fn to_string(&self) -> String {
			self.0.to_string()
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
		NewUnchecked
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

	/// SuggestedPacketNumDl
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "suggestedPacketNumDl"
	///  ],
	///  "properties": {
	///    "suggestedPacketNumDl": {
	///      "type": "integer",
	///      "minimum": 1.0
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
	pub struct SuggestedPacketNumDl {
		#[serde(rename = "suggestedPacketNumDl")]
		#[default(_code = "unsafe {std::num::NonZeroU64::new_unchecked(1)}")]
		pub suggested_packet_num_dl: std::num::NonZeroU64,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&SuggestedPacketNumDl> for SuggestedPacketNumDl {
		fn from(value: &SuggestedPacketNumDl) -> Self {
			value.clone()
		}
	}

	/// List of Supis.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of Supis.",
	///  "type": "object",
	///  "required": [
	///    "supiList"
	///  ],
	///  "properties": {
	///    "supiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SupiInfo {
		#[serde(rename = "supiList")]
		pub supi_list: Vec<Supi>,
	}

	impl From<&SupiInfo> for SupiInfo {
		fn from(value: &SupiInfo) -> Self {
			value.clone()
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
		NewUnchecked
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
		NewUnchecked
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

	/// TraceDataResponse
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "sharedTraceDataId": {
	///      "$ref": "#/components/schemas/SharedDataId"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TraceDataResponse {
		#[serde(
			rename = "sharedTraceDataId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub shared_trace_data_id: Option<SharedDataId>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
	}

	impl From<&TraceDataResponse> for TraceDataResponse {
		fn from(value: &TraceDataResponse) -> Self {
			value.clone()
		}
	}

	/// TriggerRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "failedPcscf": {
	///      "$ref": "#/components/schemas/PcscfAddress"
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
	pub struct TriggerRequest {
		#[serde(
			rename = "failedPcscf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failed_pcscf: Option<PcscfAddress>,
		pub supi: Supi,
	}

	impl From<&TriggerRequest> for TriggerRequest {
		fn from(value: &TriggerRequest) -> Self {
			value.clone()
		}
	}

	/// Indicates the purpose of the user consent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the purpose of the user consent.",
	///  "type": "string",
	///  "enum": [
	///    "ANALYTICS",
	///    "MODEL_TRAINING",
	///    "NW_CAP_EXPOSURE",
	///    "EDGEAPP_UE_LOCATION"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum UcPurpose {
		#[default]
		#[serde(rename = "ANALYTICS")]
		Analytics,
		#[serde(rename = "MODEL_TRAINING")]
		ModelTraining,
		#[serde(rename = "NW_CAP_EXPOSURE")]
		NwCapExposure,
		#[serde(rename = "EDGEAPP_UE_LOCATION")]
		EdgeappUeLocation,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UcPurpose> for UcPurpose {
		fn from(value: &UcPurpose) -> Self {
			value.clone()
		}
	}

	impl ToString for UcPurpose {
		fn to_string(&self) -> String {
			match *self {
				Self::Analytics => "ANALYTICS".to_string(),
				Self::ModelTraining => "MODEL_TRAINING".to_string(),
				Self::NwCapExposure => "NW_CAP_EXPOSURE".to_string(),
				Self::EdgeappUeLocation => "EDGEAPP_UE_LOCATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UcPurpose {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ANALYTICS" => Ok(Self::Analytics),
				"MODEL_TRAINING" => Ok(Self::ModelTraining),
				"NW_CAP_EXPOSURE" => Ok(Self::NwCapExposure),
				"EDGEAPP_UE_LOCATION" => Ok(Self::EdgeappUeLocation),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UcPurpose {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UcPurpose {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UcPurpose {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the User Consent Subscription Data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the User Consent Subscription Data.",
	///  "type": "object",
	///  "properties": {
	///    "userConsentPerPurposeList": {
	///      "description": "A map(list of key-value pairs) where user consent
	/// purpose serves as key of user consent",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/UserConsent"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UcSubscriptionData {
		/// A map(list of key-value pairs) where user consent purpose serves as
		/// key of user consent
		#[serde(
			rename = "userConsentPerPurposeList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub user_consent_per_purpose_list: ::std::collections::HashMap<String, UserConsent>,
	}

	impl From<&UcSubscriptionData> for UcSubscriptionData {
		fn from(value: &UcSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// UeContextInAmfData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "amfInfo": {
	///      "description": "AMF information",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfInfo"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "epsInterworkingInfo": {
	///      "$ref": "#/components/schemas/schemas-EpsInterworkingInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextInAmfData {
		/// AMF information
		#[serde(rename = "amfInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub amf_info: Vec<AmfInfo>,
		#[serde(
			rename = "epsInterworkingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_info: Option<SchemasEpsInterworkingInfo>,
	}

	impl From<&UeContextInAmfData> for UeContextInAmfData {
		fn from(value: &UeContextInAmfData) -> Self {
			value.clone()
		}
	}

	/// UeContextInSmfData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "emergencyInfo": {
	///      "$ref": "#/components/schemas/EmergencyInfo"
	///    },
	///    "pduSessions": {
	///      "description": "A map (list of key-value pairs where PduSessionId
	/// serves as key) of PduSessions",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PduSession"
	///      }
	///    },
	///    "pgwInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PgwInfo"
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
	pub struct UeContextInSmfData {
		#[serde(
			rename = "emergencyInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub emergency_info: Option<EmergencyInfo>,
		/// A map (list of key-value pairs where PduSessionId serves as key) of
		/// PduSessions
		#[serde(
			rename = "pduSessions",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pdu_sessions: ::std::collections::HashMap<String, PduSession>,
		#[serde(rename = "pgwInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pgw_info: Vec<PgwInfo>,
	}

	impl From<&UeContextInSmfData> for UeContextInSmfData {
		fn from(value: &UeContextInSmfData) -> Self {
			value.clone()
		}
	}

	/// UE Context In Smf Data Subscription Filter.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE Context In Smf Data Subscription Filter.",
	///  "type": "object",
	///  "properties": {
	///    "dnnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "emergencyInd": {
	///      "default": false,
	///      "type": "boolean"
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
	pub struct UeContextInSmfDataSubFilter {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "emergencyInd", default)]
		pub emergency_ind: bool,
		#[serde(rename = "snssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_list: Vec<Snssai>,
	}

	impl From<&UeContextInSmfDataSubFilter> for UeContextInSmfDataSubFilter {
		fn from(value: &UeContextInSmfDataSubFilter) -> Self {
			value.clone()
		}
	}

	/// UeContextInSmsfData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "smsfInfo3GppAccess": {
	///      "$ref": "#/components/schemas/SmsfInfo"
	///    },
	///    "smsfInfoNon3GppAccess": {
	///      "$ref": "#/components/schemas/SmsfInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextInSmsfData {
		#[serde(
			rename = "smsfInfo3GppAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_info3_gpp_access: Option<SmsfInfo>,
		#[serde(
			rename = "smsfInfoNon3GppAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_info_non3_gpp_access: Option<SmsfInfo>,
	}

	impl From<&UeContextInSmsfData> for UeContextInSmsfData {
		fn from(value: &UeContextInSmsfData) -> Self {
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

	/// UeId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "gpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
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
	pub struct UeId {
		#[serde(rename = "gpsiList", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_list: Vec<Gpsi>,
		pub supi: Supi,
	}

	impl From<&UeId> for UeId {
		fn from(value: &UeId) -> Self {
			value.clone()
		}
	}

	/// A map(list of key-value pairs) where Gpsi serves as key of arrays of
	/// Supi
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A map(list of key-value pairs) where Gpsi serves as key
	/// of arrays of Supi",
	///  "type": "object",
	///  "minProperties": 1,
	///  "additionalProperties": {
	///    "$ref": "#/components/schemas/SupiInfo"
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeIdentifiers(pub ::std::collections::HashMap<String, SupiInfo>);

	impl ::std::ops::Deref for UeIdentifiers {
		type Target = ::std::collections::HashMap<String, SupiInfo>;
		fn deref(&self) -> &::std::collections::HashMap<String, SupiInfo> {
			&self.0
		}
	}

	impl From<UeIdentifiers> for ::std::collections::HashMap<String, SupiInfo> {
		fn from(value: UeIdentifiers) -> Self {
			value.0
		}
	}

	impl From<&UeIdentifiers> for UeIdentifiers {
		fn from(value: &UeIdentifiers) -> Self {
			value.clone()
		}
	}

	impl From<::std::collections::HashMap<String, SupiInfo>> for UeIdentifiers {
		fn from(value: ::std::collections::HashMap<String, SupiInfo>) -> Self {
			Self(value)
		}
	}

	/// Represents UE information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents UE information.",
	///  "type": "object",
	///  "properties": {
	///    "5gSrvccInfo": {
	///      "$ref": "#/components/schemas/5GSrvccInfo"
	///    },
	///    "tadsInfo": {
	///      "$ref": "#/components/schemas/UeContextInfo"
	///    },
	///    "userState": {
	///      "$ref": "#/components/schemas/5GsUserState"
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
			rename = "5gSrvccInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_srvcc_info: Option<_5gSrvccInfo>,
		#[serde(rename = "tadsInfo", default, skip_serializing_if = "Option::is_none")]
		pub tads_info: Option<UeContextInfo>,
		#[serde(rename = "userState", default, skip_serializing_if = "Option::is_none")]
		pub user_state: Option<_5gsUserState>,
	}

	impl From<&UeInfo> for UeInfo {
		fn from(value: &UeInfo) -> Self {
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

	/// UE Reachable Indication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE Reachable Indication",
	///  "type": "string",
	///  "enum": [
	///    "REACHABLE",
	///    "NOT_REACHABLE",
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
	pub enum UeReachableInd {
		#[default]
		#[serde(rename = "REACHABLE")]
		Reachable,
		#[serde(rename = "NOT_REACHABLE")]
		NotReachable,
		#[serde(rename = "UNKNOWN")]
		Unknown,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeReachableInd> for UeReachableInd {
		fn from(value: &UeReachableInd) -> Self {
			value.clone()
		}
	}

	impl ToString for UeReachableInd {
		fn to_string(&self) -> String {
			match *self {
				Self::Reachable => "REACHABLE".to_string(),
				Self::NotReachable => "NOT_REACHABLE".to_string(),
				Self::Unknown => "UNKNOWN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeReachableInd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REACHABLE" => Ok(Self::Reachable),
				"NOT_REACHABLE" => Ok(Self::NotReachable),
				"UNKNOWN" => Ok(Self::Unknown),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeReachableInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeReachableInd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeReachableInd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// UeUsageType
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
	pub struct UeUsageType(pub i64);

	impl ::std::ops::Deref for UeUsageType {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<UeUsageType> for i64 {
		fn from(value: UeUsageType) -> Self {
			value.0
		}
	}

	impl From<&UeUsageType> for UeUsageType {
		fn from(value: &UeUsageType) -> Self {
			value.clone()
		}
	}

	impl From<i64> for UeUsageType {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UeUsageType {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UeUsageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeUsageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeUsageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UeUsageType {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// UnrelatedClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "defaultUnrelatedClass"
	///  ],
	///  "properties": {
	///    "defaultUnrelatedClass": {
	///      "$ref": "#/components/schemas/DefaultUnrelatedClass"
	///    },
	///    "externalUnrelatedClass": {
	///      "$ref": "#/components/schemas/ExternalUnrelatedClass"
	///    },
	///    "serviceTypeUnrelatedClasses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceTypeUnrelatedClass"
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
	pub struct UnrelatedClass {
		#[serde(rename = "defaultUnrelatedClass")]
		pub default_unrelated_class: DefaultUnrelatedClass,
		#[serde(
			rename = "externalUnrelatedClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub external_unrelated_class: Option<ExternalUnrelatedClass>,
		#[serde(
			rename = "serviceTypeUnrelatedClasses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub service_type_unrelated_classes: Vec<ServiceTypeUnrelatedClass>,
	}

	impl From<&UnrelatedClass> for UnrelatedClass {
		fn from(value: &UnrelatedClass) -> Self {
			value.clone()
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
	/// Data or the smart_default::SmartDefault configured NSSAI or the disaster
	/// roaming parameters).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains UE parameters update data set (e.g., the
	/// updated Routing ID Data or the smart_default::SmartDefault configured NSSAI or the disaster
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
	///      "$ref": "#/components/schemas/schemas-SecuredPacket"
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
		pub sec_packet: Option<SchemasSecuredPacket>,
	}

	impl From<&UpuData> for UpuData {
		fn from(value: &UpuData) -> Self {
			value.clone()
		}
	}

	/// UpuInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "provisioningTime"
	///  ],
	///  "properties": {
	///    "counterUpu": {
	///      "$ref": "#/components/schemas/CounterUpu"
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
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
	///    "upuMacIausf": {
	///      "$ref": "#/components/schemas/UpuMac"
	///    },
	///    "upuRegInd": {
	///      "$ref": "#/components/schemas/UpuRegInd"
	///    },
	///    "upuTransparentContainer": {
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
			rename = "counterUpu",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub counter_upu: Option<CounterUpu>,
		#[serde(rename = "provisioningTime")]
		pub provisioning_time: DateTime,
		#[serde(rename = "upuAckInd", default, skip_serializing_if = "Option::is_none")]
		pub upu_ack_ind: Option<UpuAckInd>,
		#[serde(rename = "upuDataList", default, skip_serializing_if = "Vec::is_empty")]
		pub upu_data_list: Vec<UpuData>,
		#[serde(
			rename = "upuMacIausf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upu_mac_iausf: Option<UpuMac>,
		#[serde(rename = "upuRegInd", default, skip_serializing_if = "Option::is_none")]
		pub upu_reg_ind: Option<UpuRegInd>,
		#[serde(
			rename = "upuTransparentContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upu_transparent_container: Option<Bytes>,
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
		NewUnchecked
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

	/// UpuRegInd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpuRegInd(pub bool);

	impl ::std::ops::Deref for UpuRegInd {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<UpuRegInd> for bool {
		fn from(value: UpuRegInd) -> Self {
			value.0
		}
	}

	impl From<&UpuRegInd> for UpuRegInd {
		fn from(value: &UpuRegInd) -> Self {
			value.clone()
		}
	}

	impl From<bool> for UpuRegInd {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UpuRegInd {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UpuRegInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpuRegInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpuRegInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UpuRegInd {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// UpuTransparentContainer
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
	pub struct UpuTransparentContainer(pub Bytes);

	impl ::std::ops::Deref for UpuTransparentContainer {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UpuTransparentContainer> for Bytes {
		fn from(value: UpuTransparentContainer) -> Self {
			value.0
		}
	}

	impl From<&UpuTransparentContainer> for UpuTransparentContainer {
		fn from(value: &UpuTransparentContainer) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UpuTransparentContainer {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UpuTransparentContainer {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UpuTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpuTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpuTransparentContainer {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UpuTransparentContainer {
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

	/// UserConsent
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CONSENT_NOT_GIVEN",
	///    "CONSENT_GIVEN"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum UserConsent {
		#[default]
		#[serde(rename = "CONSENT_NOT_GIVEN")]
		ConsentNotGiven,
		#[serde(rename = "CONSENT_GIVEN")]
		ConsentGiven,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UserConsent> for UserConsent {
		fn from(value: &UserConsent) -> Self {
			value.clone()
		}
	}

	impl ToString for UserConsent {
		fn to_string(&self) -> String {
			match *self {
				Self::ConsentNotGiven => "CONSENT_NOT_GIVEN".to_string(),
				Self::ConsentGiven => "CONSENT_GIVEN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UserConsent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CONSENT_NOT_GIVEN" => Ok(Self::ConsentNotGiven),
				"CONSENT_GIVEN" => Ok(Self::ConsentGiven),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UserConsent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the user identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the user identifier.",
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
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
	pub struct UserIdentifier {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		pub supi: Supi,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&UserIdentifier> for UserIdentifier {
		fn from(value: &UserIdentifier) -> Self {
			value.clone()
		}
	}

	/// V2xSubscriptionData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "ltePc5Ambr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "lteV2xServicesAuth": {
	///      "$ref": "#/components/schemas/LteV2xAuth"
	///    },
	///    "nrUePc5Ambr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "nrV2xServicesAuth": {
	///      "$ref": "#/components/schemas/NrV2xAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct V2xSubscriptionData {
		#[serde(
			rename = "ltePc5Ambr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lte_pc5_ambr: Option<BitRate>,
		#[serde(
			rename = "lteV2xServicesAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lte_v2x_services_auth: Option<LteV2xAuth>,
		#[serde(
			rename = "nrUePc5Ambr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_ue_pc5_ambr: Option<BitRate>,
		#[serde(
			rename = "nrV2xServicesAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_v2x_services_auth: Option<NrV2xAuth>,
	}

	impl From<&V2xSubscriptionData> for V2xSubscriptionData {
		fn from(value: &V2xSubscriptionData) -> Self {
			value.clone()
		}
	}

	/// ValidTimePeriod
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "endTime": {
	///      "$ref": "#/components/schemas/DateTime"
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
	pub struct ValidTimePeriod {
		#[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
		pub end_time: Option<DateTime>,
		#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
		pub start_time: Option<DateTime>,
	}

	impl From<&ValidTimePeriod> for ValidTimePeriod {
		fn from(value: &ValidTimePeriod) -> Self {
			value.clone()
		}
	}

	/// VgmlcAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "vgmlcAddressIpv4": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "vgmlcAddressIpv6": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "vgmlcFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VgmlcAddress {
		#[serde(
			rename = "vgmlcAddressIpv4",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vgmlc_address_ipv4: Option<Ipv4Addr>,
		#[serde(
			rename = "vgmlcAddressIpv6",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vgmlc_address_ipv6: Option<Ipv6Addr>,
		#[serde(rename = "vgmlcFqdn", default, skip_serializing_if = "Option::is_none")]
		pub vgmlc_fqdn: Option<Fqdn>,
	}

	impl From<&VgmlcAddress> for VgmlcAddress {
		fn from(value: &VgmlcAddress) -> Self {
			value.clone()
		}
	}

	/// VnGroupData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "appDescriptors": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AppDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "pduSessionTypes": {
	///      "$ref": "#/components/schemas/PduSessionTypes"
	///    },
	///    "singleNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VnGroupData {
		#[serde(
			rename = "appDescriptors",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub app_descriptors: Vec<AppDescriptor>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_types: Option<PduSessionTypes>,
		#[serde(
			rename = "singleNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub single_nssai: Option<Snssai>,
	}

	impl From<&VnGroupData> for VnGroupData {
		fn from(value: &VnGroupData) -> Self {
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

	/// Xres
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{8,32}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked
	)]
	pub struct Xres(String);

	impl ::std::ops::Deref for Xres {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Xres> for String {
		fn from(value: Xres) -> Self {
			value.0
		}
	}

	impl From<&Xres> for Xres {
		fn from(value: &Xres) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Xres {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{8,32}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{8,32}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Xres {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Xres {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Xres {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Xres {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// XresStar
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
		NewUnchecked
	)]
	pub struct XresStar(String);

	impl ::std::ops::Deref for XresStar {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<XresStar> for String {
		fn from(value: XresStar) -> Self {
			value.0
		}
	}

	impl From<&XresStar> for XresStar {
		fn from(value: &XresStar) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for XresStar {
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

	impl ::std::convert::TryFrom<&str> for XresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for XresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for XresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for XresStar {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Contains RAND, XRES, AUTN, CK, and IK
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains RAND, XRES, AUTN, CK, and IK",
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "ck",
	///    "ik",
	///    "rand",
	///    "xres"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "ck": {
	///      "$ref": "#/components/schemas/ConfidentialityKey"
	///    },
	///    "ik": {
	///      "$ref": "#/components/schemas/IntegrityKey"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    },
	///    "xres": {
	///      "$ref": "#/components/schemas/Xres"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _3gAkaAv {
		pub autn: Autn,
		pub ck: ConfidentialityKey,
		pub ik: IntegrityKey,
		pub rand: Rand,
		pub xres: Xres,
	}

	impl From<&_3gAkaAv> for _3gAkaAv {
		fn from(value: &_3gAkaAv) -> Self {
			value.clone()
		}
	}

	/// _3gppChargingCharacteristics
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
	pub struct _3gppChargingCharacteristics(pub String);

	impl ::std::ops::Deref for _3gppChargingCharacteristics {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<_3gppChargingCharacteristics> for String {
		fn from(value: _3gppChargingCharacteristics) -> Self {
			value.0
		}
	}

	impl From<&_3gppChargingCharacteristics> for _3gppChargingCharacteristics {
		fn from(value: &_3gppChargingCharacteristics) -> Self {
			value.clone()
		}
	}

	impl From<String> for _3gppChargingCharacteristics {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _3gppChargingCharacteristics {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for _3gppChargingCharacteristics {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents 5G SRVCC information for a UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents 5G SRVCC information for a UE.",
	///  "type": "object",
	///  "required": [
	///    "ue5GSrvccCapability"
	///  ],
	///  "properties": {
	///    "cMsisdn": {
	///      "$ref": "#/components/schemas/CMsisdn"
	///    },
	///    "stnSr": {
	///      "$ref": "#/components/schemas/StnSr"
	///    },
	///    "ue5GSrvccCapability": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5gSrvccInfo {
		#[serde(rename = "cMsisdn", default, skip_serializing_if = "Option::is_none")]
		pub c_msisdn: Option<CMsisdn>,
		#[serde(rename = "stnSr", default, skip_serializing_if = "Option::is_none")]
		pub stn_sr: Option<StnSr>,
		#[serde(rename = "ue5GSrvccCapability")]
		pub ue5_g_srvcc_capability: bool,
	}

	impl From<&_5gSrvccInfo> for _5gSrvccInfo {
		fn from(value: &_5gSrvccInfo) -> Self {
			value.clone()
		}
	}

	/// _5gVnGroupConfiguration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "5gVnGroupData": {
	///      "$ref": "#/components/schemas/5GVnGroupData"
	///    },
	///    "afInstanceId": {
	///      "type": "string"
	///    },
	///    "internalGroupIdentifier": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "members": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "mtcProviderInformation": {
	///      "$ref": "#/components/schemas/MtcProviderInformation"
	///    },
	///    "referenceId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Model5GvnGroupConfiguration {
		#[serde(
			rename = "afInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_instance_id: Option<String>,
		#[serde(
			rename = "5gVnGroupData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_vn_group_data: Option<_5gVnGroupData>,
		#[serde(
			rename = "internalGroupIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub internal_group_identifier: Option<GroupId>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub members: Vec<Gpsi>,
		#[serde(
			rename = "mtcProviderInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mtc_provider_information: Option<MtcProviderInformation>,
		#[serde(
			rename = "referenceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reference_id: Option<ReferenceId>,
	}

	impl From<&Model5GvnGroupConfiguration> for Model5GvnGroupConfiguration {
		fn from(value: &Model5GvnGroupConfiguration) -> Self {
			value.clone()
		}
	}

	/// _5gVnGroupData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "additionalDnAaaAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-IpAddress"
	///      },
	///      "minItems": 1
	///    },
	///    "appDescriptors": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-AppDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnAaaAddress": {
	///      "$ref": "#/components/schemas/schemas-IpAddress"
	///    },
	///    "dnAaaFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "dnAaaIpAddressAllocation": {
	///      "type": "boolean"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "pduSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "secondaryAuth": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5gVnGroupData {
		#[serde(
			rename = "additionalDnAaaAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_dn_aaa_addresses: Vec<SchemasIpAddress>,
		#[serde(
			rename = "appDescriptors",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub app_descriptors: Vec<SchemasAppDescriptor>,
		#[serde(
			rename = "dnAaaAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_aaa_address: Option<SchemasIpAddress>,
		#[serde(rename = "dnAaaFqdn", default, skip_serializing_if = "Option::is_none")]
		pub dn_aaa_fqdn: Option<Fqdn>,
		#[serde(
			rename = "dnAaaIpAddressAllocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_aaa_ip_address_allocation: Option<bool>,
		pub dnn: Dnn,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_types: Vec<PduSessionType>,
		#[serde(rename = "sNssai")]
		pub s_nssai: Snssai,
		#[serde(
			rename = "secondaryAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_auth: Option<bool>,
	}

	impl From<&_5gVnGroupData> for _5gVnGroupData {
		fn from(value: &_5gVnGroupData) -> Self {
			value.clone()
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

	/// _5mbsAuthorizationInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "5mbsSessionIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsSessionId"
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
	pub struct _5mbsAuthorizationInfo(pub Option<_5mbsAuthorizationInfoInner>);

	impl ::std::ops::Deref for _5mbsAuthorizationInfo {
		type Target = Option<_5mbsAuthorizationInfoInner>;
		fn deref(&self) -> &Option<_5mbsAuthorizationInfoInner> {
			&self.0
		}
	}

	impl From<_5mbsAuthorizationInfo> for Option<_5mbsAuthorizationInfoInner> {
		fn from(value: _5mbsAuthorizationInfo) -> Self {
			value.0
		}
	}

	impl From<&_5mbsAuthorizationInfo> for _5mbsAuthorizationInfo {
		fn from(value: &_5mbsAuthorizationInfo) -> Self {
			value.clone()
		}
	}

	impl From<Option<_5mbsAuthorizationInfoInner>> for _5mbsAuthorizationInfo {
		fn from(value: Option<_5mbsAuthorizationInfoInner>) -> Self {
			Self(value)
		}
	}

	/// _5mbsAuthorizationInfoInner
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "5mbsSessionIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsSessionId"
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
	pub struct _5mbsAuthorizationInfoInner {
		#[serde(
			rename = "5mbsSessionIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub five_mbs_session_ids: Vec<MbsSessionId>,
	}

	impl From<&_5mbsAuthorizationInfoInner> for _5mbsAuthorizationInfoInner {
		fn from(value: &_5mbsAuthorizationInfoInner) -> Self {
			value.clone()
		}
	}
}
