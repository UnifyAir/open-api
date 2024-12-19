#![allow(unused_qualifications)]

pub use oasbi::nrf::types::{self as models, *};
use serde_with::skip_serializing_none;

use crate::types::*;

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AccessTokenRequestHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BootstrappingInfoRequestHeaderParams {
	pub if_none_match: Option<String>,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrieveCompleteSearchHeaderParams {
	pub accept_encoding: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrieveCompleteSearchPathParams {
	/// Id of a stored search
	pub search_id: String,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScpDomainRoutingInfoUnsubscribePathParams {
	/// Unique ID of the subscription to remove
	pub subscription_id: String,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeregisterNfInstancePathParams {
	/// Unique ID of the NF Instance to deregister
	pub nf_instance_id: uuid::Uuid,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNfInstancePathParams {
	/// Unique ID of the NF Instance
	pub nf_instance_id: uuid::Uuid,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNfInstanceQueryParams {
	/// Features supported by the NF Service Consumer
	#[serde(rename = "requester-features")]
	#[validate(
		regex(path = *RE_GETNFINSTANCEQUERYPARAMS_REQUESTER_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETNFINSTANCEQUERYPARAMS_REQUESTER_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegisterNfInstanceHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegisterNfInstancePathParams {
	/// Unique ID of the NF Instance to register
	pub nf_instance_id: uuid::Uuid,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNfInstanceHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
	pub if_match: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNfInstancePathParams {
	/// Unique ID of the NF Instance to update
	pub nf_instance_id: uuid::Uuid,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNfInstancesQueryParams {
	/// Type of NF
	#[serde(rename = "nf-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nf_type: Option<models::NfType>,
	/// How many items to return at one time
	#[serde(rename = "limit")]
	#[validate(range(min = 1_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<i32>,
	/// Page number where the response shall start
	#[serde(rename = "page-number")]
	#[validate(range(min = 1_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_number: Option<i32>,
	/// Maximum number of items in each returned page
	#[serde(rename = "page-size")]
	#[validate(range(min = 1_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_size: Option<i32>,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SearchNfInstancesHeaderParams {
	pub accept_encoding: Option<String>,
	pub if_none_match: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate, smart_default::SmartDefault)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SearchNfInstancesQueryParams {
	/// Type of the target NF
	#[serde(rename = "target-nf-type")]
	pub target_nf_type: models::NfType,
	/// Type of the requester NF
	#[serde(rename = "requester-nf-type")]
	pub requester_nf_type: models::NfType,
	/// collocated NF types that candidate NFs should preferentially support
	#[serde(rename = "preferred-collocated-nf-types")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_collocated_nf_types: Option<Vec<models::CollocatedNfType>>,
	/// NfInstanceId of the requester NF
	#[serde(rename = "requester-nf-instance-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_nf_instance_id: Option<uuid::Uuid>,
	/// Names of the services offered by the NF
	#[serde(rename = "service-names")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_names: Option<Vec<models::ServiceName>>,
	/// FQDN of the requester NF
	#[serde(rename = "requester-nf-instance-fqdn")]
	#[validate(
		length(min = 4, max = 253),
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_REQUESTER_NF_INSTANCE_FQDN),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_nf_instance_fqdn: Option<String>,
	/// Id of the PLMN of either the target NF, or in SNPN scenario the
	/// Credentials Holder in the PLMN
	#[serde(rename = "target-plmn-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_plmn_list: Option<Vec<models::PlmnId>>,
	/// Id of the PLMN where the NF issuing the Discovery request is located
	#[serde(rename = "requester-plmn-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_plmn_list: Option<Vec<models::PlmnId>>,
	/// Identity of the NF instance being discovered
	#[serde(rename = "target-nf-instance-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_nf_instance_id: Option<uuid::Uuid>,
	/// FQDN of the NF instance being discovered
	#[serde(rename = "target-nf-fqdn")]
	#[validate(
		length(min = 4, max = 253),
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_TARGET_NF_FQDN),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_nf_fqdn: Option<String>,
	/// Uri of the home NRF
	#[serde(rename = "hnrf-uri")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hnrf_uri: Option<String>,
	/// Slice info of the target NF
	#[serde(rename = "snssais")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub snssais: Option<Vec<models::Snssai>>,
	/// Slice info of the requester NF
	#[serde(rename = "requester-snssais")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_snssais: Option<Vec<models::ExtSnssai>>,
	/// PLMN specific Slice info of the target NF
	#[serde(rename = "plmn-specific-snssai-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_specific_snssai_list: Option<Vec<models::SchemasPlmnSnssai>>,
	/// PLMN-specific slice info of the NF issuing the Discovery request
	#[serde(rename = "requester-plmn-specific-snssai-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_plmn_specific_snssai_list: Option<Vec<models::SchemasPlmnSnssai>>,
	/// Dnn supported by the BSF, SMF or UPF
	#[serde(rename = "dnn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dnn: Option<String>,
	/// The IPv4 Index supported by the candidate UPF.
	#[serde(rename = "ipv4-index")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipv4_index: Option<models::IpIndex>,
	/// The IPv6 Index supported by the candidate UPF.
	#[serde(rename = "ipv6-index")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipv6_index: Option<models::IpIndex>,
	/// NSI IDs that are served by the services being discovered
	#[serde(rename = "nsi-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsi_list: Option<Vec<String>>,
	#[serde(rename = "smf-serving-area")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smf_serving_area: Option<String>,
	#[serde(rename = "mbsmf-serving-area")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mbsmf_serving_area: Option<String>,
	/// Tracking Area Identity
	#[serde(rename = "tai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tai: Option<crate::types::Object>,
	/// AMF Region Identity
	#[serde(rename = "amf-region-id")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_AMF_REGION_ID),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub amf_region_id: Option<String>,
	/// AMF Set Identity
	#[serde(rename = "amf-set-id")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_AMF_SET_ID),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub amf_set_id: Option<String>,
	/// Guami used to search for an appropriate AMF
	#[serde(rename = "guami")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guami: Option<Guami>,
	/// SUPI of the user
	#[serde(rename = "supi")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_SUPI),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supi: Option<String>,
	/// IPv4 address of the UE
	#[serde(rename = "ue-ipv4-address")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_UE_IPV4_ADDRESS),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ue_ipv4_address: Option<String>,
	/// IP domain of the UE, which supported by BSF
	#[serde(rename = "ip-domain")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ip_domain: Option<String>,
	/// IPv6 prefix of the UE
	#[serde(rename = "ue-ipv6-prefix")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_UE_IPV6_PREFIX),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ue_ipv6_prefix: Option<String>,
	/// Combined PGW-C and SMF or a standalone SMF
	#[serde(rename = "pgw-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pgw_ind: Option<bool>,
	/// Indicates combined PGW-C+SMF or standalone SMF are preferred
	#[serde(rename = "preferred-pgw-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_pgw_ind: Option<bool>,
	/// PGW FQDN of a combined PGW-C and SMF
	#[serde(rename = "pgw")]
	#[validate(
		length(min = 4, max = 253),
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_PGW),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pgw: Option<String>,
	/// PGW IP Address of a combined PGW-C and SMF
	#[serde(rename = "pgw-ip")]
	#[serde(deserialize_with = "deserialize_optional_nullable")]
	#[serde(default = "default_optional_nullable")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pgw_ip: Option<Nullable<models::IpAddr>>,
	/// GPSI of the user
	#[serde(rename = "gpsi")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_GPSI),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gpsi: Option<String>,
	/// external group identifier of the user
	#[serde(rename = "external-group-identity")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_EXTERNAL_GROUP_IDENTITY),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub external_group_identity: Option<String>,
	/// internal group identifier of the user
	#[serde(rename = "internal-group-identity")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_INTERNAL_GROUP_IDENTITY),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub internal_group_identity: Option<String>,
	/// PFD data
	#[serde(rename = "pfd-data")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pfd_data: Option<crate::types::Object>,
	/// data set supported by the NF
	#[serde(rename = "data-set")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_set: Option<models::DataSetId>,
	/// routing indicator in SUCI
	#[serde(rename = "routing-indicator")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_ROUTING_INDICATOR),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub routing_indicator: Option<String>,
	/// Group IDs of the NFs being discovered
	#[serde(rename = "group-id-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_id_list: Option<Vec<models::NfGroupId>>,
	/// Data network access identifiers of the NFs being discovered
	#[serde(rename = "dnai-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dnai_list: Option<Vec<models::Dnai>>,
	/// list of PDU Session Type required to be supported by the target NF
	#[serde(rename = "pdu-session-types")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pdu_session_types: Option<Vec<models::PduSessionType>>,
	/// Analytics event(s) requested to be supported by the Nnwdaf_AnalyticsInfo
	/// service
	#[serde(rename = "event-id-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id_list: Option<Vec<models::EventId>>,
	/// Analytics event(s) requested to be supported by the
	/// Nnwdaf_EventsSubscription service.
	#[serde(rename = "nwdaf-event-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nwdaf_event_list: Option<Vec<models::NwdafEvent>>,
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// UPF supporting interworking with EPS or not
	#[serde(rename = "upf-iwk-eps-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub upf_iwk_eps_ind: Option<bool>,
	/// PLMN ID supported by a CHF
	#[serde(rename = "chf-supported-plmn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chf_supported_plmn: Option<crate::types::Object>,
	/// preferred target NF location
	#[serde(rename = "preferred-locality")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_locality: Option<String>,
	/// AccessType supported by the target NF
	#[serde(rename = "access-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub access_type: Option<models::AccessType>,
	/// Maximum number of NFProfiles to return in the response
	#[serde(rename = "limit")]
	#[validate(range(min = 1_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<i32>,
	/// Features required to be supported by the target NF
	#[serde(rename = "required-features")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_features: Option<Vec<models::SupportedFeatures>>,
	/// the complex query condition expression
	#[serde(rename = "complex-query")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub complex_query: Option<models::ComplexQuery>,
	/// Maximum payload size of the response expressed in kilo octets
	#[serde(rename = "max-payload-size")]
	#[validate(range(max = 2000_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_payload_size: Option<i32>,
	/// Extended query for maximum payload size of the response expressed in
	/// kilo octets
	#[serde(rename = "max-payload-size-ext")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_payload_size_ext: Option<i32>,
	/// ATSSS Capability
	#[serde(rename = "atsss-capability")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub atsss_capability: Option<crate::types::Object>,
	/// UPF supporting allocating UE IP addresses/prefixes
	#[serde(rename = "upf-ue-ip-addr-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub upf_ue_ip_addr_ind: Option<bool>,
	/// Requested client type served by the NF
	#[serde(rename = "client-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_type: Option<models::ExternalClientType>,
	/// LMF identification to be discovered
	#[serde(rename = "lmf-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lmf_id: Option<String>,
	/// Requested AN node type served by the NF
	#[serde(rename = "an-node-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub an_node_type: Option<models::AnNodeType>,
	/// Requested RAT type served by the NF
	#[serde(rename = "rat-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rat_type: Option<models::RatType>,
	/// preferred Tracking Area Identity
	#[serde(rename = "preferred-tai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_tai: Option<crate::types::Object>,
	/// preferred NF Instances
	#[serde(rename = "preferred-nf-instances")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_nf_instances: Option<Vec<models::NfInstanceId>>,
	/// Target SNPN Identity, or the Credentials Holder in the SNPN
	#[serde(rename = "target-snpn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_snpn: Option<crate::types::Object>,
	/// SNPN ID(s) of the NF instance issuing the Discovery request
	#[serde(rename = "requester-snpn-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_snpn_list: Option<Vec<models::PlmnIdNid>>,
	/// NEF exposured by the AF
	#[serde(rename = "af-ee-data")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub af_ee_data: Option<crate::types::Object>,
	/// UPF collocated with W-AGF
	#[serde(rename = "w-agf-info")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub w_agf_info: Option<crate::types::Object>,
	/// UPF collocated with TNGF
	#[serde(rename = "tngf-info")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tngf_info: Option<crate::types::Object>,
	/// UPF collocated with TWIF
	#[serde(rename = "twif-info")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub twif_info: Option<crate::types::Object>,
	/// Target NF Set ID
	#[serde(rename = "target-nf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_nf_set_id: Option<String>,
	/// Target NF Service Set ID
	#[serde(rename = "target-nf-service-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_nf_service_set_id: Option<String>,
	/// NEF ID
	#[serde(rename = "nef-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nef_id: Option<String>,
	/// Notification Type
	#[serde(rename = "notification-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub notification_type: Option<models::NotificationType>,
	/// N1 Message Class
	#[serde(rename = "n1-msg-class")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n1_msg_class: Option<models::N1MessageClass>,
	/// N2 Information Class
	#[serde(rename = "n2-info-class")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n2_info_class: Option<models::N2InformationClass>,
	/// areas that can be served by the target NF
	#[serde(rename = "serving-scope")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serving_scope: Option<Vec<String>>,
	/// IMSI of the requester UE to search for an appropriate NF (e.g. HSS)
	#[serde(rename = "imsi")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_IMSI),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub imsi: Option<String>,
	/// IMPI of the requester UE to search for a target HSS
	#[serde(rename = "ims-private-identity")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ims_private_identity: Option<String>,
	/// IMS Public Identity of the requester UE to search for a target HSS
	#[serde(rename = "ims-public-identity")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ims_public_identity: Option<String>,
	/// MSISDN of the requester UE to search for a target HSS
	#[serde(rename = "msisdn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub msisdn: Option<String>,
	/// Preferred API version of the services to be discovered
	#[serde(rename = "preferred-api-versions")]
	#[validate()]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_api_versions: Option<std::collections::HashMap<String, String>>,
	/// PCF supports V2X
	#[serde(rename = "v2x-support-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub v2x_support_ind: Option<bool>,
	/// UPF supports redundant gtp-u to be discovered
	#[serde(rename = "redundant-gtpu")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redundant_gtpu: Option<bool>,
	/// UPF supports redundant transport path to be discovered
	#[serde(rename = "redundant-transport")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redundant_transport: Option<bool>,
	/// UPF which is configured for IPUPS functionality to be discovered
	#[serde(rename = "ipups")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipups: Option<bool>,
	/// SCP domains the target SCP or SEPP belongs to
	#[serde(rename = "scp-domain-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scp_domain_list: Option<Vec<String>>,
	/// Address domain reachable through the SCP
	#[serde(rename = "address-domain")]
	#[validate(
		length(min = 4, max = 253),
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_ADDRESS_DOMAIN),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address_domain: Option<String>,
	/// IPv4 address reachable through the SCP
	#[serde(rename = "ipv4-addr")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_IPV4_ADDR),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipv4_addr: Option<String>,
	/// IPv6 prefix reachable through the SCP
	#[serde(rename = "ipv6-prefix")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_IPV6_PREFIX),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipv6_prefix: Option<String>,
	/// NF Set ID served by the SCP
	#[serde(rename = "served-nf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub served_nf_set_id: Option<String>,
	/// Id of the PLMN reachable through the SCP or SEPP
	#[serde(rename = "remote-plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remote_plmn_id: Option<crate::types::Object>,
	/// Id of the SNPN reachable through the SCP or SEPP
	#[serde(rename = "remote-snpn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remote_snpn_id: Option<crate::types::Object>,
	/// UPF Instance(s) configured for data forwarding are requested
	#[serde(rename = "data-forwarding")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_forwarding: Option<bool>,
	/// NF Instance(s) serving the full PLMN are preferred
	#[serde(rename = "preferred-full-plmn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_full_plmn: Option<bool>,
	/// Features supported by the NF Service Consumer that is invoking the
	/// Nnrf_NFDiscovery service
	#[serde(rename = "requester-features")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_REQUESTER_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requester_features: Option<String>,
	/// realm-id to search for an appropriate UDSF
	#[serde(rename = "realm-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub realm_id: Option<String>,
	/// storage-id to search for an appropriate UDSF
	#[serde(rename = "storage-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub storage_id: Option<String>,
	/// V-SMF capability supported by the target NF instance(s)
	#[serde(rename = "vsmf-support-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vsmf_support_ind: Option<bool>,
	/// I-SMF capability supported by the target NF instance(s)
	#[serde(rename = "ismf-support-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ismf_support_ind: Option<bool>,
	/// Uri of the NRF holding the NF profile of a target NF Instance
	#[serde(rename = "nrf-disc-uri")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nrf_disc_uri: Option<String>,
	/// Preferred vendor specific features of the services to be discovered
	#[serde(rename = "preferred-vendor-specific-features")]
	#[validate()]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_vendor_specific_features: Option<
		std::collections::HashMap<
			String,
			std::collections::HashMap<String, Vec<models::VendorSpecificFeature>>,
		>,
	>,
	/// Preferred vendor specific features of the network function to be
	/// discovered
	#[serde(rename = "preferred-vendor-specific-nf-features")]
	#[validate()]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_vendor_specific_nf_features:
		Option<std::collections::HashMap<String, Vec<models::VendorSpecificFeature>>>,
	/// PFCP features required to be supported by the target UPF
	#[serde(rename = "required-pfcp-features")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_pfcp_features: Option<String>,
	/// Indicates the Home Network Public Key ID which shall be able to be
	/// served by the NF instance
	#[serde(rename = "home-pub-key-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub home_pub_key_id: Option<i32>,
	/// PCF supports ProSe Capability
	#[serde(rename = "prose-support-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prose_support_ind: Option<bool>,
	/// analytics aggregation is supported by NWDAF or not
	#[serde(rename = "analytics-aggregation-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub analytics_aggregation_ind: Option<bool>,
	/// NF Set Id served by target NF
	#[serde(rename = "serving-nf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serving_nf_set_id: Option<String>,
	/// NF type served by the target NF
	#[serde(rename = "serving-nf-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serving_nf_type: Option<models::NfType>,
	/// Lisf of ML Analytics Filter information of Nnwdaf_MLModelProvision
	/// service
	#[serde(rename = "ml-analytics-info-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ml_analytics_info_list: Option<Vec<models::SchemasMlAnalyticsInfo>>,
	/// analytics matadata provisioning is supported by NWDAF or not
	#[serde(rename = "analytics-metadata-prov-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub analytics_metadata_prov_ind: Option<bool>,
	/// the service capability supported by the target NSACF
	#[serde(rename = "nsacf-capability")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsacf_capability: Option<crate::types::Object>,
	/// List of MBS Session ID(s)
	#[serde(rename = "mbs-session-id-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mbs_session_id_list: Option<Vec<models::MbsSessionId>>,
	/// Area Session ID
	#[serde(rename = "area-session-id")]
	#[validate(range(min = 0_i32, max = 65535_i32))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub area_session_id: Option<i32>,
	/// The GMLC Number supported by the GMLC
	#[serde(rename = "gmlc-number")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_GMLC_NUMBER),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gmlc_number: Option<String>,
	/// N6 IP address of PSA UPF supported by the EASDF
	#[serde(rename = "upf-n6-ip")]
	#[serde(deserialize_with = "deserialize_optional_nullable")]
	#[serde(default = "default_optional_nullable")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub upf_n6_ip: Option<Nullable<models::IpAddr>>,
	/// Tracking Area Identifiers of the NFs being discovered
	#[serde(rename = "tai-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tai_list: Option<Vec<models::Tai>>,
	/// Indicates the precedence of the preference query parameters (from higher
	/// to lower)
	#[serde(rename = "preferences-precedence")]
	#[validate(length(min = 2))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferences_precedence: Option<Vec<String>>,
	/// Indicating the support for onboarding.
	#[serde(rename = "support-onboarding-capability")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub support_onboarding_capability: Option<bool>,
	/// UAS NF functionality is supported by NEF or not
	#[serde(rename = "uas-nf-functionality-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uas_nf_functionality_ind: Option<bool>,
	/// indicates the V2X capability that the target PCF needs to support.
	#[serde(rename = "v2x-capability")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub v2x_capability: Option<crate::types::Object>,
	/// indicates the ProSe capability that the target PCF needs to support.
	#[serde(rename = "prose-capability")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prose_capability: Option<crate::types::Object>,
	/// Identifier of shared data stored in the NF being discovered
	#[serde(rename = "shared-data-id")]
	#[validate(
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_SHARED_DATA_ID),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shared_data_id: Option<String>,
	/// Home Network Identifier query.
	#[serde(rename = "target-hni")]
	#[validate(
		length(min = 4, max = 253),
		regex(path = *RE_SEARCHNFINSTANCESQUERYPARAMS_TARGET_HNI),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_hni: Option<String>,
	/// Resolution of the identity of the target PLMN based on the GPSI of the
	/// UE
	#[serde(rename = "target-nw-resolution")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_nw_resolution: Option<bool>,
	/// NF Instance IDs to be excluded from the NF Discovery procedure
	#[serde(rename = "exclude-nfinst-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_nfinst_list: Option<Vec<models::NfInstanceId>>,
	/// NF service instance IDs to be excluded from the NF Discovery procedure
	#[serde(rename = "exclude-nfservinst-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_nfservinst_list: Option<Vec<models::NfServiceInstance>>,
	/// NF Service Set IDs to be excluded from the NF Discovery procedure
	#[serde(rename = "exclude-nfserviceset-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_nfserviceset_list: Option<Vec<models::NfServiceSetId>>,
	/// NF Set IDs to be excluded from the NF Discovery procedure
	#[serde(rename = "exclude-nfset-list")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_nfset_list: Option<Vec<models::NfSetId>>,
	/// Preferred analytics delays supported by the NWDAF to be discovered
	#[serde(rename = "preferred-analytics-delays")]
	#[validate()]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_analytics_delays: Option<std::collections::HashMap<String, models::DurationSec>>,
	/// Indicating the support for High Latency communication.
	#[serde(rename = "high-latency-com")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub high_latency_com: Option<bool>,
	/// NSAC Service Area Identifier
	#[serde(rename = "nsac-sai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsac_sai: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_REQUESTER_NF_INSTANCE_FQDN: regex::Regex = regex::Regex::new(r"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_TARGET_NF_FQDN: regex::Regex = regex::Regex::new(r"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_AMF_REGION_ID: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]{2}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_AMF_SET_ID: regex::Regex = regex::Regex::new(r"^[0-3][A-Fa-f0-9]{2}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_UE_IPV4_ADDRESS: regex::Regex = regex::Regex::new(r"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_UE_IPV6_PREFIX: regex::Regex = regex::Regex::new(r"(?&#x3D;.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))(/(([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?&#x3D;.*^((([^:]+:){7}([^:]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(/.+)$)").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_PGW: regex::Regex = regex::Regex::new(r"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_GPSI: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_EXTERNAL_GROUP_IDENTITY: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_INTERNAL_GROUP_IDENTITY: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,10}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_ROUTING_INDICATOR: regex::Regex = regex::Regex::new(r"^[0-9]{1,4}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_IMSI: regex::Regex = regex::Regex::new(r"^[0-9]{5,15}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_ADDRESS_DOMAIN: regex::Regex = regex::Regex::new(r"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_IPV4_ADDR: regex::Regex = regex::Regex::new(r"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_IPV6_PREFIX: regex::Regex = regex::Regex::new(r"(?&#x3D;.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))(/(([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?&#x3D;.*^((([^:]+:){7}([^:]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(/.+)$)").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_REQUESTER_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_GMLC_NUMBER: regex::Regex = regex::Regex::new(r"^[0-9]{5,15}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_SHARED_DATA_ID: regex::Regex = regex::Regex::new(r"^[0-9]{5,6}-.+$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_SEARCHNFINSTANCESQUERYPARAMS_TARGET_HNI: regex::Regex = regex::Regex::new(r"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$").unwrap();
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SCpDomainRoutingInfoGetHeaderParams {
	pub accept_encoding: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SCpDomainRoutingInfoGetQueryParams {
	/// Indication of local SCP Domain Routing Information
	#[serde(rename = "local")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local: Option<bool>,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScpDomainRoutingInfoSubscribeHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrieveStoredSearchHeaderParams {
	pub accept_encoding: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrieveStoredSearchPathParams {
	/// Id of a stored search
	pub search_id: String,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemoveSubscriptionPathParams {
	/// Unique ID of the subscription to remove
	#[validate(
		regex(path = *RE_REMOVESUBSCRIPTIONPATHPARAMS_SUBSCRIPTION_ID),
	)]
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_REMOVESUBSCRIPTIONPATHPARAMS_SUBSCRIPTION_ID: regex::Regex = regex::Regex::new(r"^([0-9]{5,6}-(x3Lf57A:nid&#x3D;[A-Fa-f0-9]{11}:)?)?[^-]+$").unwrap();
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSubscriptionHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
}

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSubscriptionPathParams {
	/// Unique ID of the subscription to update
	#[validate(
		regex(path = *RE_UPDATESUBSCRIPTIONPATHPARAMS_SUBSCRIPTION_ID),
	)]
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATESUBSCRIPTIONPATHPARAMS_SUBSCRIPTION_ID: regex::Regex = regex::Regex::new(r"^([0-9]{5,6}-(x3Lf57A:nid&#x3D;[A-Fa-f0-9]{11}:)?)?[^-]+$").unwrap();
}

#[skip_serializing_none]
#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateSubscriptionHeaderParams {
	pub content_encoding: Option<String>,
	pub accept_encoding: Option<String>,
}
