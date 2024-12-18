use crate::nrf::types::*;

/// Information of an NF Instance registered in the NRF
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Information of an NF Instance registered in the NRF",
///  "type": "object",
///  "anyOf": [
///    {
///      "required": [
///        "fqdn"
///      ]
///    },
///    {
///      "required": [
///        "ipv4Addresses"
///      ]
///    },
///    {
///      "required": [
///        "ipv6Addresses"
///      ]
///    }
///  ],
///  "required": [
///    "nfInstanceId",
///    "nfStatus",
///    "nfType"
///  ],
///  "properties": {
///    "5gDdnmfInfo": {
///      "$ref": "#/components/schemas/5GDdnmfInfo"
///    },
///    "aanfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of AanfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/AanfInfo"
///      }
///    },
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
///    "amfInfo": {
///      "$ref": "#/components/schemas/AmfInfo"
///    },
///    "amfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of AmfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/AmfInfo"
///      }
///    },
///    "ausfInfo": {
///      "$ref": "#/components/schemas/AusfInfo"
///    },
///    "ausfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of AusfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/AusfInfo"
///      }
///    },
///    "bsfInfo": {
///      "$ref": "#/components/schemas/BsfInfo"
///    },
///    "bsfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of BsfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/BsfInfo"
///      }
///    },
///    "capacity": {
///      "type": "integer",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    },
///    "chfInfo": {
///      "$ref": "#/components/schemas/ChfInfo"
///    },
///    "chfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of ChfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/ChfInfo"
///      }
///    },
///    "collocatedNfInstances": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/CollocatedNfInstance"
///      },
///      "minItems": 1
///    },
///    "customInfo": {
///      "type": "object"
///    },
///    "dccfInfo": {
///      "$ref": "#/components/schemas/DccfInfo"
///    },
///    "defaultNotificationSubscriptions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/DefaultNotificationSubscription"
///      }
///    },
///    "easdfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of EasdfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/EasdfInfo"
///      }
///    },
///    "fqdn": {
///      "$ref": "#/components/schemas/Fqdn"
///    },
///    "gmlcInfo": {
///      "$ref": "#/components/schemas/GmlcInfo"
///    },
///    "heartBeatTimer": {
///      "type": "integer",
///      "minimum": 1.0
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
///        "$ref": "#/components/schemas/HssInfo"
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
///      "$ref": "#/components/schemas/IwmscInfo"
///    },
///    "lcHSupportInd": {
///      "default": false,
///      "type": "boolean"
///    },
///    "lmfInfo": {
///      "$ref": "#/components/schemas/LmfInfo"
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
///        "$ref": "#/components/schemas/MbSmfInfo"
///      }
///    },
///    "mbUpfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of MbUpfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/MbUpfInfo"
///      }
///    },
///    "mfafInfo": {
///      "$ref": "#/components/schemas/MfafInfo"
///    },
///    "mnpfInfo": {
///      "$ref": "#/components/schemas/MnpfInfo"
///    },
///    "nefInfo": {
///      "$ref": "#/components/schemas/NefInfo"
///    },
///    "nfInstanceId": {
///      "$ref": "#/components/schemas/NfInstanceId"
///    },
///    "nfInstanceName": {
///      "type": "string"
///    },
///    "nfProfileChangesInd": {
///      "default": false,
///      "readOnly": true,
///      "type": "boolean"
///    },
///    "nfProfileChangesSupportInd": {
///      "default": false,
///      "writeOnly": true,
///      "type": "boolean"
///    },
///    "nfServiceList": {
///      "description": "A map (list of key-value pairs) where
/// serviceInstanceId serves as key of NFService\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/NFService1"
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
///        "$ref": "#/components/schemas/NFService1"
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
///    "nrfInfo": {
///      "$ref": "#/components/schemas/NrfInfo"
///    },
///    "nsacfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of NsacfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/NsacfInfo"
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
///      "$ref": "#/components/schemas/NssaafInfo"
///    },
///    "nwdafInfo": {
///      "$ref": "#/components/schemas/NwdafInfo"
///    },
///    "nwdafInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of NwdafInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/NwdafInfo"
///      }
///    },
///    "olcHSupportInd": {
///      "default": false,
///      "type": "boolean"
///    },
///    "pcfInfo": {
///      "$ref": "#/components/schemas/PcfInfo"
///    },
///    "pcfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of PcfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/PcfInfo"
///      }
///    },
///    "pcscfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of PcscfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/PcscfInfo"
///      }
///    },
///    "perPlmnSnssaiList": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PlmnSnssai"
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
///      "$ref": "#/components/schemas/ScpInfo"
///    },
///    "seppInfo": {
///      "$ref": "#/components/schemas/SeppInfo"
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
///      "$ref": "#/components/schemas/SmfInfo"
///    },
///    "smfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of SmfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/SmfInfo"
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
///      "$ref": "#/components/schemas/TrustAfInfo"
///    },
///    "tsctsfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of TsctsfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/TsctsfInfo"
///      }
///    },
///    "udmInfo": {
///      "$ref": "#/components/schemas/UdmInfo"
///    },
///    "udmInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of UdmInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/UdmInfo"
///      }
///    },
///    "udrInfo": {
///      "$ref": "#/components/schemas/UdrInfo"
///    },
///    "udrInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of UdrInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/UdrInfo"
///      }
///    },
///    "udsfInfo": {
///      "$ref": "#/components/schemas/UdsfInfo"
///    },
///    "udsfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of UdsfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/UdsfInfo"
///      }
///    },
///    "upfInfo": {
///      "$ref": "#/components/schemas/UpfInfo"
///    },
///    "upfInfoList": {
///      "description": "A map (list of key-value pairs) where a (unique)
/// valid JSON string serves as key of UpfInfo\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "$ref": "#/components/schemas/UpfInfo"
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
pub struct NfProfile1Unchecked {
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of AanfInfo
	fqdn: Option<Fqdn>,
	#[serde(rename = "ipv4Addresses")]
	ipv4_addresses: Vec<Ipv4Addr>,
	#[serde(rename = "ipv6Addresses")]
	ipv6_addresses: Vec<Ipv6Addr>,
	#[serde(
		rename = "aanfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	aanf_info_list: ::std::collections::HashMap<String, AanfInfo>,
	#[serde(
		rename = "allowedNfDomains",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	allowed_nf_domains: Vec<String>,
	#[serde(
		rename = "allowedNfTypes",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	allowed_nf_types: Vec<NfType>,
	#[serde(
		rename = "allowedNssais",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	allowed_nssais: Vec<ExtSnssai>,
	#[serde(
		rename = "allowedPlmns",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	allowed_plmns: Vec<PlmnId>,
	#[serde(
		rename = "allowedSnpns",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	allowed_snpns: Vec<PlmnIdNid>,
	#[serde(rename = "amfInfo", default, skip_serializing_if = "Option::is_none")]
	amf_info: Option<AmfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of AmfInfo
	#[serde(
		rename = "amfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	amf_info_list: ::std::collections::HashMap<String, AmfInfo>,
	#[serde(rename = "ausfInfo", default, skip_serializing_if = "Option::is_none")]
	ausf_info: Option<AusfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of AusfInfo
	#[serde(
		rename = "ausfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	ausf_info_list: ::std::collections::HashMap<String, AusfInfo>,
	#[serde(rename = "bsfInfo", default, skip_serializing_if = "Option::is_none")]
	bsf_info: Option<BsfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of BsfInfo
	#[serde(
		rename = "bsfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	bsf_info_list: ::std::collections::HashMap<String, BsfInfo>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	capacity: Option<u16>,
	#[serde(rename = "chfInfo", default, skip_serializing_if = "Option::is_none")]
	chf_info: Option<ChfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of ChfInfo
	#[serde(
		rename = "chfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	chf_info_list: ::std::collections::HashMap<String, ChfInfo>,
	#[serde(
		rename = "collocatedNfInstances",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	collocated_nf_instances: Vec<CollocatedNfInstance>,
	#[serde(
		rename = "customInfo",
		default,
		skip_serializing_if = "::serde_json::Map::is_empty"
	)]
	custom_info: ::serde_json::Map<String, ::serde_json::Value>,
	#[serde(rename = "dccfInfo", default, skip_serializing_if = "Option::is_none")]
	dccf_info: Option<DccfInfo>,
	#[serde(
		rename = "defaultNotificationSubscriptions",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	default_notification_subscriptions: Vec<DefaultNotificationSubscription>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of EasdfInfo
	#[serde(
		rename = "easdfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	easdf_info_list: ::std::collections::HashMap<String, EasdfInfo>,
	#[serde(
		rename = "5gDdnmfInfo",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	five_g_ddnmf_info: Option<_5gDdnmfInfo>,
	#[serde(rename = "gmlcInfo", default, skip_serializing_if = "Option::is_none")]
	gmlc_info: Option<GmlcInfo>,
	#[serde(
		rename = "heartBeatTimer",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	heart_beat_timer: Option<std::num::NonZeroU64>,
	#[serde(rename = "hniList", default, skip_serializing_if = "Vec::is_empty")]
	hni_list: Vec<Fqdn>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of HssInfo
	#[serde(
		rename = "hssInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	hss_info_list: ::std::collections::HashMap<String, HssInfo>,
	#[serde(
		rename = "interPlmnFqdn",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	inter_plmn_fqdn: Option<Fqdn>,
	#[serde(rename = "iwmscInfo", default, skip_serializing_if = "Option::is_none")]
	iwmsc_info: Option<IwmscInfo>,
	#[serde(rename = "lcHSupportInd", default)]
	lc_h_support_ind: bool,
	#[serde(rename = "lmfInfo", default, skip_serializing_if = "Option::is_none")]
	lmf_info: Option<LmfInfo>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	load: Option<i64>,
	#[serde(
		rename = "loadTimeStamp",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	load_time_stamp: Option<DateTime>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	locality: Option<String>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of MbSmfInfo
	#[serde(
		rename = "mbSmfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	mb_smf_info_list: ::std::collections::HashMap<String, MbSmfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of MbUpfInfo
	#[serde(
		rename = "mbUpfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	mb_upf_info_list: ::std::collections::HashMap<String, MbUpfInfo>,
	#[serde(rename = "mfafInfo", default, skip_serializing_if = "Option::is_none")]
	mfaf_info: Option<MfafInfo>,
	#[serde(rename = "mnpfInfo", default, skip_serializing_if = "Option::is_none")]
	mnpf_info: Option<MnpfInfo>,
	#[serde(rename = "nefInfo", default, skip_serializing_if = "Option::is_none")]
	nef_info: Option<NefInfo>,
	#[serde(rename = "nfInstanceId")]
	nf_instance_id: NfInstanceId,
	#[serde(
		rename = "nfInstanceName",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	nf_instance_name: Option<String>,
	#[serde(rename = "nfProfileChangesInd", default)]
	nf_profile_changes_ind: bool,
	#[serde(rename = "nfProfileChangesSupportInd", default)]
	nf_profile_changes_support_ind: bool,
	/// A map (list of key-value pairs) where serviceInstanceId serves
	/// as key of NFService
	#[serde(
		rename = "nfServiceList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	nf_service_list: ::std::collections::HashMap<String, NfService1>,
	#[serde(rename = "nfServicePersistence", default)]
	nf_service_persistence: bool,
	#[serde(rename = "nfServices", default, skip_serializing_if = "Vec::is_empty")]
	nf_services: Vec<NfService1>,
	#[serde(rename = "nfSetIdList", default, skip_serializing_if = "Vec::is_empty")]
	nf_set_id_list: Vec<NfSetId>,
	/// A map (list of key-value pairs) where NfSetId serves as key of
	/// DateTime
	#[serde(
		rename = "nfSetRecoveryTimeList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	nf_set_recovery_time_list: ::std::collections::HashMap<String, DateTime>,
	#[serde(rename = "nfStatus")]
	nf_status: NfStatus,
	#[serde(rename = "nfType")]
	nf_type: NfType,
	#[serde(rename = "nrfInfo", default, skip_serializing_if = "Option::is_none")]
	nrf_info: Option<NrfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of NsacfInfo
	#[serde(
		rename = "nsacfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	nsacf_info_list: ::std::collections::HashMap<String, NsacfInfo>,
	#[serde(rename = "nsiList", default, skip_serializing_if = "Vec::is_empty")]
	nsi_list: Vec<String>,
	#[serde(
		rename = "nssaafInfo",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	nssaaf_info: Option<NssaafInfo>,
	#[serde(rename = "nwdafInfo", default, skip_serializing_if = "Option::is_none")]
	nwdaf_info: Option<NwdafInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of NwdafInfo
	#[serde(
		rename = "nwdafInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	nwdaf_info_list: ::std::collections::HashMap<String, NwdafInfo>,
	#[serde(rename = "olcHSupportInd", default)]
	olc_h_support_ind: bool,
	#[serde(rename = "pcfInfo", default, skip_serializing_if = "Option::is_none")]
	pcf_info: Option<PcfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of PcfInfo
	#[serde(
		rename = "pcfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	pcf_info_list: ::std::collections::HashMap<String, PcfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of PcscfInfo
	#[serde(
		rename = "pcscfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	pcscf_info_list: ::std::collections::HashMap<String, PcscfInfo>,
	#[serde(
		rename = "perPlmnSnssaiList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	per_plmn_snssai_list: Vec<PlmnSnssai>,
	#[serde(rename = "plmnList", default, skip_serializing_if = "Vec::is_empty")]
	plmn_list: Vec<PlmnId>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	priority: Option<u16>,
	#[serde(
		rename = "recoveryTime",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	recovery_time: Option<DateTime>,
	#[serde(rename = "sNssais", default, skip_serializing_if = "Vec::is_empty")]
	s_nssais: Vec<ExtSnssai>,
	#[serde(rename = "scpDomains", default, skip_serializing_if = "Vec::is_empty")]
	scp_domains: Vec<String>,
	#[serde(rename = "scpInfo", default, skip_serializing_if = "Option::is_none")]
	scp_info: Option<ScpInfo>,
	#[serde(rename = "seppInfo", default, skip_serializing_if = "Option::is_none")]
	sepp_info: Option<SeppInfo>,
	/// A map (list of key-value pairs) where NfServiceSetId serves as
	/// key of DateTime
	#[serde(
		rename = "serviceSetRecoveryTimeList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	service_set_recovery_time_list: ::std::collections::HashMap<String, DateTime>,
	#[serde(
		rename = "servingScope",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	serving_scope: Vec<String>,
	#[serde(rename = "smfInfo", default, skip_serializing_if = "Option::is_none")]
	smf_info: Option<SmfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of SmfInfo
	#[serde(
		rename = "smfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	smf_info_list: ::std::collections::HashMap<String, SmfInfo>,
	#[serde(rename = "snpnList", default, skip_serializing_if = "Vec::is_empty")]
	snpn_list: Vec<PlmnIdNid>,
	/// The key of the map is the IANA-assigned SMI Network Management
	/// Private Enterprise Codes
	#[serde(
		rename = "supportedVendorSpecificFeatures",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	supported_vendor_specific_features:
		::std::collections::HashMap<String, Vec<VendorSpecificFeature>>,
	#[serde(
		rename = "trustAfInfo",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	trust_af_info: Option<TrustAfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of TsctsfInfo
	#[serde(
		rename = "tsctsfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	tsctsf_info_list: ::std::collections::HashMap<String, TsctsfInfo>,
	#[serde(rename = "udmInfo", default, skip_serializing_if = "Option::is_none")]
	udm_info: Option<UdmInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of UdmInfo
	#[serde(
		rename = "udmInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	udm_info_list: ::std::collections::HashMap<String, UdmInfo>,
	#[serde(rename = "udrInfo", default, skip_serializing_if = "Option::is_none")]
	udr_info: Option<UdrInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of UdrInfo
	#[serde(
		rename = "udrInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	udr_info_list: ::std::collections::HashMap<String, UdrInfo>,
	#[serde(rename = "udsfInfo", default, skip_serializing_if = "Option::is_none")]
	udsf_info: Option<UdsfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of UdsfInfo
	#[serde(
		rename = "udsfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	udsf_info_list: ::std::collections::HashMap<String, UdsfInfo>,
	#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
	upf_info: Option<UpfInfo>,
	/// A map (list of key-value pairs) where a (unique) valid JSON
	/// string serves as key of UpfInfo
	#[serde(
		rename = "upfInfoList",
		default,
		skip_serializing_if = "::std::collections::HashMap::is_empty"
	)]
	upf_info_list: ::std::collections::HashMap<String, UpfInfo>,
	#[serde(rename = "vendorId", default, skip_serializing_if = "Option::is_none")]
	vendor_id: Option<VendorId>,
}

impl From<&NfProfile1Unchecked> for NfProfile1Unchecked {
	fn from(value: &NfProfile1Unchecked) -> Self {
		value.clone()
	}
}

#[derive(
	::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
)]
#[serde(try_from = "NfProfile1Unchecked")]
pub struct NfProfile1(NfProfile1Unchecked);

impl TryFrom<NfProfile1Unchecked> for NfProfile1 {
	type Error = error::ConversionError;

	fn try_from(value: NfProfile1Unchecked) -> Result<Self, Self::Error> {
		// Checking the following condition from the NFProfile1Unchecked
		//anyOf:
		//  - required: [ fqdn ]
		//	- required: [ ipv4Addresses ]
		//	- required: [ ipv6Addresses ]
		if value.fqdn.is_none() && value.ipv4_addresses.len() == 0 && value.ipv6_addresses.len() == 0 {
			Err("Atleast one of of fqdn, ipv4_addresses, ipv6_addresses is absent".into())
		} else {
			Ok(Self (value))
		}
	}
}


