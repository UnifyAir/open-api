#![allow(unused_qualifications)]

use http::HeaderValue;
pub use oasbi::udm::types::{self as models, *};
use oasbi::{DeserResponse, ReqError};
use serde_with::skip_serializing_none;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GET3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GET3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get3GppRegistrationQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GET3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GET3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNon3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETNON3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETNON3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNon3GppRegistrationQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETNON3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETNON3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Call3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CALL3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CALL3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Non3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_NON3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_NON3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAmDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAmDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETAMDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETAMDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAmDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETAMDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// Serving PLMN ID or SNPN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
	/// List of PLMNs adjacent to the UE's serving PLMN
	#[serde(rename = "adjacent-plmns")]
	#[validate(length(min = 1))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub adjacent_plmns: Option<Vec<models::PlmnId>>,
	/// Indication whether Disaster Roaming service is applied or not
	#[serde(rename = "disaster-roaming-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disaster_roaming_ind: Option<bool>,
}

lazy_static::lazy_static! {
	static ref RE_GETAMDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthorizeNiddDataPathParams {
	/// Represents the scope of the UE for which the NIDD configuration are
	/// authorized. Contains the GPSI of the user or the external group ID.
	#[validate(
		regex(path = *RE_AUTHORIZENIDDDATAPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
}

lazy_static::lazy_static! {
	static ref RE_AUTHORIZENIDDDATAPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Create5GMbsGroupPathParams {
	/// External Identifier of the Group
	#[validate(
		regex(path = *RE_CREATE5GMBSGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CREATE5GMBSGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Delete5GMbsGroupPathParams {
	/// External Identifier of the Group
	#[validate(
		regex(path = *RE_DELETE5GMBSGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_DELETE5GMBSGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get5GMbsGroupPathParams {
	/// External Identifier of the group
	#[validate(
		regex(path = *RE_GET5GMBSGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GET5GMBSGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Modify5GMbsGroupPathParams {
	/// External Identifier of the group
	#[validate(
		regex(path = *RE_MODIFY5GMBSGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_MODIFY5GMBSGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Modify5GMbsGroupQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_MODIFY5GMBSGROUPQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_MODIFY5GMBSGROUPQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Create5GVnGroupPathParams {
	/// External Identifier of the Group
	#[validate(
		regex(path = *RE_CREATE5GVNGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CREATE5GVNGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Delete5GVnGroupPathParams {
	/// External Identifier of the Group
	#[validate(
		regex(path = *RE_DELETE5GVNGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_DELETE5GVNGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Delete5GVnGroupQueryParams {
	/// MTC Provider Information that originated the service operation
	#[serde(rename = "mtc-provider-info")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mtc_provider_info: Option<String>,
	/// AF ID that originated the service operation
	#[serde(rename = "af-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub af_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get5GVnGroupPathParams {
	/// External Identifier of the group
	#[validate(
		regex(path = *RE_GET5GVNGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GET5GVNGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Modify5GVnGroupPathParams {
	/// External Identifier of the group
	#[validate(
		regex(path = *RE_MODIFY5GVNGROUPPATHPARAMS_EXT_GROUP_ID),
	)]
	pub ext_group_id: String,
}

lazy_static::lazy_static! {
	static ref RE_MODIFY5GVNGROUPPATHPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Modify5GVnGroupQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_MODIFY5GVNGROUPQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_MODIFY5GVNGROUPQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMbsDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMbsDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETMBSDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETMBSDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMbsDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETMBSDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETMBSDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConfirmAuthPathParams {
	/// SUPI of the user
	#[validate(
		regex(path = *RE_CONFIRMAUTHPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_CONFIRMAUTHPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateEeSubscriptionPathParams {
	/// Represents the scope of the UE for which the subscription is applied.
	/// Contains the GPSI of the user or the external group ID or any UE.
	#[validate(
		regex(path = *RE_CREATEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
}

lazy_static::lazy_static! {
	static ref RE_CREATEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+|anyUE)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAuthPathParams {
	/// SUPI of the user
	#[validate(
		regex(path = *RE_DELETEAUTHPATHPARAMS_SUPI),
	)]
	pub supi: String,
	/// authEvent Id
	pub auth_event_id: String,
}

lazy_static::lazy_static! {
	static ref RE_DELETEAUTHPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteEeSubscriptionPathParams {
	/// Represents the scope of the UE for which the subscription is applied.
	/// Contains the GPSI of the user or the external group ID or any UE.
	#[validate(
		regex(path = *RE_DELETEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
	/// Id of the EE Subscription
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_DELETEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+|anyUE)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetEcrDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetEcrDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETECRDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETECRDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetEcrDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETECRDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETECRDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSupiOrGpsiHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSupiOrGpsiPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSUPIORGPSIPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSUPIORGPSIPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSupiOrGpsiQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSUPIORGPSIQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// AF identifier
	#[serde(rename = "af-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub af_id: Option<String>,
	/// Application port identifier
	#[serde(rename = "app-port-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub app_port_id: Option<crate::types::Object>,
	/// AF Service Identifier
	#[serde(rename = "af-service-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub af_service_id: Option<String>,
	/// MTC Provider Information
	#[serde(rename = "mtc-provider-info")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mtc_provider_info: Option<String>,
	/// Requested GPSI Type
	#[serde(rename = "requested-gpsi-type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requested_gpsi_type: Option<models::GpsiType>,
}

lazy_static::lazy_static! {
	static ref RE_GETSUPIORGPSIQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateAuthDataPathParams {
	/// SUPI or SUCI of the user
	#[validate(
		regex(path = *RE_GENERATEAUTHDATAPATHPARAMS_SUPI_OR_SUCI),
	)]
	pub supi_or_suci: String,
}

lazy_static::lazy_static! {
	static ref RE_GENERATEAUTHDATAPATHPARAMS_SUPI_OR_SUCI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.*|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateGbaAvPathParams {
	/// SUPI of the user
	#[validate(
		regex(path = *RE_GENERATEGBAAVPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GENERATEGBAAVPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateAvPathParams {
	/// SUPI of the user
	#[validate(
		regex(path = *RE_GENERATEAVPATHPARAMS_SUPI),
	)]
	pub supi: String,
	/// Type of AV requested by HSS
	pub hss_auth_type: models::HssAuthTypeInUri,
}

lazy_static::lazy_static! {
	static ref RE_GENERATEAVPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateProseAvPathParams {
	/// SUPI or SUCI of the user
	#[validate(
		regex(path = *RE_GENERATEPROSEAVPATHPARAMS_SUPI_OR_SUCI),
	)]
	pub supi_or_suci: String,
}

lazy_static::lazy_static! {
	static ref RE_GENERATEPROSEAVPATHPARAMS_SUPI_OR_SUCI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.*|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRgAuthDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRgAuthDataPathParams {
	/// SUPI or SUCI of the user
	#[validate(
		regex(path = *RE_GETRGAUTHDATAPATHPARAMS_SUPI_OR_SUCI),
	)]
	pub supi_or_suci: String,
}

lazy_static::lazy_static! {
	static ref RE_GETRGAUTHDATAPATHPARAMS_SUPI_OR_SUCI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.*|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRgAuthDataQueryParams {
	/// Authenticated indication
	#[serde(rename = "authenticated-ind")]
	pub authenticated_ind: bool,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETRGAUTHDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// serving PLMN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETRGAUTHDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetGroupIdentifiersHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetGroupIdentifiersQueryParams {
	/// External Group Identifier
	#[serde(rename = "ext-group-id")]
	#[validate(
		regex(path = *RE_GETGROUPIDENTIFIERSQUERYPARAMS_EXT_GROUP_ID),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ext_group_id: Option<String>,
	/// Internal Group Identifier
	#[serde(rename = "int-group-id")]
	#[validate(
		regex(path = *RE_GETGROUPIDENTIFIERSQUERYPARAMS_INT_GROUP_ID),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub int_group_id: Option<String>,
	/// Indication whether UE identifiers are required or not
	#[serde(rename = "ue-id-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ue_id_ind: Option<bool>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETGROUPIDENTIFIERSQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// AF identifier
	#[serde(rename = "af-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub af_id: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETGROUPIDENTIFIERSQUERYPARAMS_EXT_GROUP_ID: regex::Regex = regex::Regex::new(r"^extgroupid-[^@]+@[^@]+$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_GETGROUPIDENTIFIERSQUERYPARAMS_INT_GROUP_ID: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,10}$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_GETGROUPIDENTIFIERSQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IpSmGwDeregistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_IPSMGWDEREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_IPSMGWDEREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IpSmGwRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_IPSMGWREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_IPSMGWREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetIpSmGwRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETIPSMGWREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETIPSMGWREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsBcaDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsBcaDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETLCSBCADATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSBCADATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsBcaDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETLCSBCADATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSBCADATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsMoDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsMoDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETLCSMODATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSMODATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsMoDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETLCSMODATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSMODATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsPrivacyDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsPrivacyDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETLCSPRIVACYDATAPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSPRIVACYDATAPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLcsPrivacyDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETLCSPRIVACYDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETLCSPRIVACYDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMultipleIdentifiersQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETMULTIPLEIDENTIFIERSQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// list of the GPSIs
	#[serde(rename = "gpsi-list")]
	#[validate(length(min = 1))]
	pub gpsi_list: Vec<models::Gpsi>,
}

lazy_static::lazy_static! {
	static ref RE_GETMULTIPLEIDENTIFIERSQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NwdafDeregistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_NWDAFDEREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// NWDAF registration identifier
	pub nwdaf_registration_id: String,
}

lazy_static::lazy_static! {
	static ref RE_NWDAFDEREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NwdafRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_NWDAFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// NWDAF registration identifier
	pub nwdaf_registration_id: String,
}

lazy_static::lazy_static! {
	static ref RE_NWDAFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNwdafRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETNWDAFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETNWDAFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNwdafRegistrationQueryParams {
	/// List of analytics Id(s) provided by the consumers of NWDAF.
	#[serde(rename = "analytics-ids")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub analytics_ids: Option<Vec<models::EventId>>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETNWDAFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETNWDAFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PeiUpdatePathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_PEIUPDATEPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_PEIUPDATEPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreatePpDataEntryPathParams {
	/// Identifier of the UE
	// TODO: Add the regex pattern
	pub ue_id: String,
	/// Application Function Instance Identifier
	pub af_instance_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeletePpDataEntryPathParams {
	/// Identifier of the UE
	// TODO: Add the regex pattern
	pub ue_id: String,
	/// Application Function Instance Identifier
	pub af_instance_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPpDataEntryPathParams {
	/// Identifier of the UE
	// TODO: Add the regex pattern
	pub ue_id: String,
	/// Application Function Instance Identifier
	pub af_instance_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPpDataEntryQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETPPDATAENTRYQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETPPDATAENTRYQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Update3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATE3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATE3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Update3GppRegistrationQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATE3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATE3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNon3GppRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATENON3GPPREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATENON3GPPREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNon3GppRegistrationQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATENON3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATENON3GPPREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNwdafRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATENWDAFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// NWDAF registration identifier
	pub nwdaf_registration_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATENWDAFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNwdafRegistrationQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATENWDAFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATENWDAFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSmfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATESMFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Identifier of the PDU session
	#[validate(range(min = 0_i32, max = 255_i32))]
	pub pdu_session_id: i32,
}

lazy_static::lazy_static! {
	static ref RE_UPDATESMFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSmfRegistrationQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATESMFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATESMFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetProseDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetProseDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETPROSEDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETPROSEDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetProseDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETPROSEDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETPROSEDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProvideLocationInfoPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_PROVIDELOCATIONINFOPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_PROVIDELOCATIONINFOPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CAgAckPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CAGACKPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_CAGACKPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SNssaisAckPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_SNSSAISACKPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_SNSSAISACKPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SorAckInfoPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_SORACKINFOPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_SORACKINFOPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpuAckPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPUACKPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_UPUACKPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QueryUeInfoPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_QUERYUEINFOPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_QUERYUEINFOPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QueryUeInfoQueryParams {
	/// attributes to be retrieved
	#[serde(rename = "fields")]
	#[validate(length(min = 1))]
	pub fields: Vec<String>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_QUERYUEINFOQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_QUERYUEINFOQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReportSmDeliveryStatusPathParams {
	/// Represents the scope of the UE for which the Service Specific Parameters
	/// are authorized. Contains the GPSI of the user or the external group ID.
	#[validate(
		regex(path = *RE_REPORTSMDELIVERYSTATUSPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
}

lazy_static::lazy_static! {
	static ref RE_REPORTSMDELIVERYSTATUSPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDataSetsHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDataSetsPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETDATASETSPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETDATASETSPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDataSetsQueryParams {
	/// List of dataset names
	#[serde(rename = "dataset-names")]
	#[validate(length(min = 2))]
	pub dataset_names: Vec<models::DataSetName>,
	/// serving PLMN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
	/// Indication whether Disaster Roaming service is applied or not
	#[serde(rename = "disaster-roaming-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disaster_roaming_ind: Option<bool>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETDATASETSQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETDATASETSQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSharedDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSharedDataQueryParams {
	/// List of shared data ids
	#[serde(rename = "shared-data-ids")]
	#[validate(length(min = 1))]
	pub shared_data_ids: Vec<models::SharedDataId>,
	/// Supported Features; this query parameter should not be used
	#[serde(rename = "supportedFeatures")]
	#[validate(
		regex(path = *RE_GETSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES2),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features2: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}
lazy_static::lazy_static! {
	static ref RE_GETSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES2: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetIndividualSharedDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetIndividualSharedDataPathParams {
	/// Id of the Shared data
	#[validate(length(min = 1))]
	pub shared_data_id: Vec<models::SharedDataId>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetIndividualSharedDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETINDIVIDUALSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETINDIVIDUALSHAREDDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrieveSmfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_RETRIEVESMFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Identifier of the PDU session
	#[validate(range(min = 0_i32, max = 255_i32))]
	pub pdu_session_id: i32,
}

lazy_static::lazy_static! {
	static ref RE_RETRIEVESMFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateRoamingInformationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATEROAMINGINFORMATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATEROAMINGINFORMATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SmfDeregistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_SMFDEREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Identifier of the PDU session
	#[validate(range(min = 0_i32, max = 255_i32))]
	pub pdu_session_id: i32,
}

lazy_static::lazy_static! {
	static ref RE_SMFDEREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SmfDeregistrationQueryParams {
	#[serde(rename = "smf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smf_set_id: Option<String>,
	#[serde(rename = "smf-instance-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smf_instance_id: Option<uuid::Uuid>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmfSelDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmfSelDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSMFSELDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSMFSELDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmfSelDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSMFSELDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// serving PLMN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
	/// Indication whether Disaster Roaming service is applied or not
	#[serde(rename = "disaster-roaming-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disaster_roaming_ind: Option<bool>,
}

lazy_static::lazy_static! {
	static ref RE_GETSMFSELDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSMFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSMFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmfRegistrationQueryParams {
	#[serde(rename = "single-nssai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub single_nssai: Option<crate::types::Object>,
	#[serde(rename = "dnn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dnn: Option<String>,
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSMFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETSMFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_REGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Identifier of the PDU session
	#[validate(range(min = 0_i32, max = 255_i32))]
	pub pdu_session_id: i32,
}

lazy_static::lazy_static! {
	static ref RE_REGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get3GppSmsfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GET3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GET3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Get3GppSmsfRegistrationQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GET3GPPSMSFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GET3GPPSMSFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Call3GppSmsfDeregistrationHeaderParams {
	pub if_match: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Call3GppSmsfDeregistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CALL3GPPSMSFDEREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CALL3GPPSMSFDEREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Call3GppSmsfDeregistrationQueryParams {
	#[serde(rename = "smsf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smsf_set_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Non3GppSmsfDeregistrationHeaderParams {
	pub if_match: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Non3GppSmsfDeregistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_NON3GPPSMSFDEREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_NON3GPPSMSFDEREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Non3GppSmsfDeregistrationQueryParams {
	#[serde(rename = "smsf-set-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smsf_set_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNon3GppSmsfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETNON3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETNON3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNon3GppSmsfRegistrationQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETNON3GPPSMSFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETNON3GPPSMSFREGISTRATIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Call3GppSmsfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CALL3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CALL3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Non3GppSmsfRegistrationPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_NON3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_NON3GPPSMSFREGISTRATIONPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsMngtDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsMngtDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSMSMNGTDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSMSMNGTDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsMngtDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSMSMNGTDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETSMSMNGTDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSMSDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSMSDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmsDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSMSDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETSMSDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SendRoutingInfoSmPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_SENDROUTINGINFOSMPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_SENDROUTINGINFOSMPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceSpecificAuthorizationRemovalPathParams {
	/// Represents the scope of the UE for which the Service Specific
	/// configuration authorization to be removed. Contains the GPSI of the user
	/// or the external group ID.
	#[validate(
		regex(path = *RE_SERVICESPECIFICAUTHORIZATIONREMOVALPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
	/// Represents the specific service for which the Service Specific
	/// configuration authorization to be removed.
	pub service_type: models::ServiceType,
}

lazy_static::lazy_static! {
	static ref RE_SERVICESPECIFICAUTHORIZATIONREMOVALPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceSpecificAuthorizationPathParams {
	/// Represents the scope of the UE for which the Service Specific Parameters
	/// are authorized. Contains the GPSI of the user or the external group ID.
	#[validate(
		regex(path = *RE_SERVICESPECIFICAUTHORIZATIONPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
	/// Represents the specific service for which the Service Specific
	/// Parameters are authorized.
	pub service_type: models::ServiceType,
}

lazy_static::lazy_static! {
	static ref RE_SERVICESPECIFICAUTHORIZATIONPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+)$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETSMDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETSMDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETSMDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	#[serde(rename = "single-nssai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub single_nssai: Option<crate::types::Object>,
	#[serde(rename = "dnn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dnn: Option<String>,
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETSMDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNssaiHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNssaiPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETNSSAIPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETNSSAIPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNssaiQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETNSSAIQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// serving PLMN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
	/// Indication whether Disaster Roaming service is applied or not
	#[serde(rename = "disaster-roaming-ind")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disaster_roaming_ind: Option<bool>,
}

lazy_static::lazy_static! {
	static ref RE_GETNSSAIQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SubscribePathParams {
	/// Identity of the user
	#[validate(
		regex(path = *RE_SUBSCRIBEPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_SUBSCRIBEPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdatePathParams {
	/// Identifier of the UE
	// TODO: Add the regex pattern
	pub ue_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATEQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATEQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnsubscribePathParams {
	/// Identity of the user
	#[validate(
		regex(path = *RE_UNSUBSCRIBEPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Id of the SDM Subscription
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UNSUBSCRIBEPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnsubscribeForSharedDataPathParams {
	/// Id of the Shared data Subscription
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModifyPathParams {
	/// Identity of the user
	#[validate(
		regex(path = *RE_MODIFYPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
	/// Id of the SDM Subscription
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_MODIFYPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModifyQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_MODIFYQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_MODIFYQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModifySharedDataSubsPathParams {
	/// Id of the SDM Subscription
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModifySharedDataSubsQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_MODIFYSHAREDDATASUBSQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_MODIFYSHAREDDATASUBSQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTraceConfigDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTraceConfigDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETTRACECONFIGDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETTRACECONFIGDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTraceConfigDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETTRACECONFIGDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// serving PLMN ID
	#[serde(rename = "plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub plmn_id: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_GETTRACECONFIGDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeregAmfPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_DEREGAMFPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_DEREGAMFPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSorInfoPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_UPDATESORINFOPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATESORINFOPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRegistrationsPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETREGISTRATIONSPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETREGISTRATIONSPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRegistrationsQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETREGISTRATIONSQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// List of UECM registration dataset names
	#[serde(rename = "registration-dataset-names")]
	#[validate(length(min = 2))]
	pub registration_dataset_names: Vec<models::RegistrationDataSetName>,
	#[serde(rename = "single-nssai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub single_nssai: Option<crate::types::Object>,
	#[serde(rename = "dnn")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dnn: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETREGISTRATIONSQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInAmfDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETUECTXINAMFDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINAMFDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInAmfDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETUECTXINAMFDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINAMFDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInSmfDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETUECTXINSMFDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINSMFDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInSmfDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETUECTXINSMFDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINSMFDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInSmsfDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETUECTXINSMSFDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINSMSFDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUeCtxInSmsfDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETUECTXINSMSFDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETUECTXINSMSFDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLocationInfoPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETLOCATIONINFOPATHPARAMS_UE_ID),
	)]
	pub ue_id: String,
}

lazy_static::lazy_static! {
	static ref RE_GETLOCATIONINFOPATHPARAMS_UE_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetLocationInfoQueryParams {
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETLOCATIONINFOQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETLOCATIONINFOQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateEeSubscriptionPathParams {
	/// Represents the scope of the UE for which the subscription is applied.
	/// Contains the GPSI of the user or the external group ID or any UE.
	#[validate(
		regex(path = *RE_UPDATEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY),
	)]
	pub ue_identity: String,
	/// Id of the EE Subscription
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UPDATEEESUBSCRIPTIONPATHPARAMS_UE_IDENTITY: regex::Regex = regex::Regex::new(r"^(msisdn-[0-9]{5,15}|.+|extid-[^@]+@[^@]+|extgroupid-[^@]+@[^@]+|anyUE)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateEeSubscriptionQueryParams {
	/// Features required to be supported by the target NF
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_UPDATEEESUBSCRIPTIONQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_UPDATEEESUBSCRIPTIONQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUcDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUcDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETUCDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETUCDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetUcDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETUCDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// User consent purpose
	#[serde(rename = "uc-purpose")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uc_purpose: Option<models::UcPurpose>,
}

lazy_static::lazy_static! {
	static ref RE_GETUCDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetV2xDataHeaderParams {
	pub if_none_match: Option<String>,
	pub if_modified_since: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetV2xDataPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_GETV2XDATAPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
	static ref RE_GETV2XDATAPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetV2xDataQueryParams {
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_GETV2XDATAQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_GETV2XDATAQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, smart_default::SmartDefault)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(untagged)]
pub enum UpdateNwdafRegistration200Response {
	#[default]
	PatchResult(models::PatchResult),
	NwdafRegistration(models::NwdafRegistration),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(untagged)]
pub enum Modify200Response {
	PatchResult(models::PatchResult),
	SdmSubscription(models::SdmSubscription),
}
