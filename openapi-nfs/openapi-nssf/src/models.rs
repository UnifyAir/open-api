#![allow(unused_qualifications)]

pub use oasbi::nssf::types::{self as models, *};

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NSsaiAvailabilityDeletePathParams {
	/// Identifier of the NF service consumer instance
	pub nf_id: String,
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
pub struct NSsaiAvailabilityPatchHeaderParams {
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
pub struct NSsaiAvailabilityPatchPathParams {
	/// Identifier of the NF service consumer instance
	pub nf_id: String,
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
pub struct NSsaiAvailabilityPutHeaderParams {
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
pub struct NSsaiAvailabilityPutPathParams {
	/// Identifier of the NF service consumer instance
	pub nf_id: uuid::Uuid,
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
pub struct NSSelectionGetQueryParams {
	/// NF type of the NF service consumer
	#[serde(rename = "nf-type")]
	pub nf_type: models::NfType,
	/// NF Instance ID of the NF service consumer
	#[serde(rename = "nf-id")]
	pub nf_id: uuid::Uuid,
	/// Requested network slice information during Registration procedure
	#[serde(rename = "slice-info-request-for-registration")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slice_info_request_for_registration: Option<crate::types::Object>,
	/// Requested network slice information during PDU session establishment
	/// procedure
	#[serde(rename = "slice-info-request-for-pdu-session")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slice_info_request_for_pdu_session: Option<crate::types::Object>,
	/// Requested network slice information during UE confuguration update
	/// procedure
	#[serde(rename = "slice-info-request-for-ue-cu")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slice_info_request_for_ue_cu: Option<crate::types::Object>,
	/// PLMN ID of the HPLMN
	#[serde(rename = "home-plmn-id")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub home_plmn_id: Option<crate::types::Object>,
	/// TAI of the UE
	#[serde(rename = "tai")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tai: Option<crate::types::Object>,
	/// Features required to be supported by the NFs in the target slice
	/// instance
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_NSSELECTIONGETQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
}

lazy_static::lazy_static! {
	static ref RE_NSSELECTIONGETQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
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
pub struct NSsaiAvailabilitySubModifyPatchHeaderParams {
	pub content_encoding: Option<String>,
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
pub struct NSsaiAvailabilitySubModifyPatchPathParams {
	/// Identifier of the subscription for notification
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
pub struct NSsaiAvailabilityUnsubscribePathParams {
	/// Identifier of the subscription for notification
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
pub struct NSsaiAvailabilityPostHeaderParams {
	pub content_encoding: Option<String>,
}
