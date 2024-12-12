#![allow(unused_qualifications)]

use http::HeaderValue;
pub use oasbi::amf::types::{self as models, *};
use oasbi::{DeserResponse, ReqError};
use serde_with::skip_serializing_none;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::types::*;

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AMfStatusChangeSubscribeModfyPathParams {
	/// AMF Status Change Subscription Identifier
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
pub struct AMfStatusChangeUnSubscribePathParams {
	/// AMF Status Change Subscription Identifier
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
pub struct DeleteSubscriptionPathParams {
	/// Unique ID of the subscription to be deleted
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
pub struct ModifySubscriptionPathParams {
	/// Unique ID of the subscription to be modified
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
pub struct CancelLocationPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_CANCELLOCATIONPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CANCELLOCATIONPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct ProvideLocationInfoPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_PROVIDELOCATIONINFOPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_PROVIDELOCATIONINFOPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct ProvidePositioningInfoPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_PROVIDEPOSITIONINGINFOPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_PROVIDEPOSITIONINGINFOPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct CancelRelocateUeContextPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_CANCELRELOCATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CANCELRELOCATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct CreateUeContextPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_CREATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_CREATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct EBiAssignmentPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_EBIASSIGNMENTPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_EBIASSIGNMENTPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct RegistrationStatusUpdatePathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_REGISTRATIONSTATUSUPDATEPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_REGISTRATIONSTATUSUPDATEPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct ReleaseUeContextPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_RELEASEUECONTEXTPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_RELEASEUECONTEXTPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct RelocateUeContextPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_RELOCATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_RELOCATEUECONTEXTPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct UEContextTransferPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_UECONTEXTTRANSFERPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_UECONTEXTTRANSFERPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(5g-guti-[0-9]{5,6}[0-9a-fA-F]{14}|imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct N1N2MessageUnSubscribePathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_N1N2MESSAGEUNSUBSCRIBEPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
	/// Subscription Identifier
	pub subscription_id: String,
}

lazy_static::lazy_static! {
	static ref RE_N1N2MESSAGEUNSUBSCRIBEPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct N1N2MessageTransferPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_N1N2MESSAGETRANSFERPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_N1N2MESSAGETRANSFERPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|cid-.{1,255}|.+)$").unwrap();
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
pub struct N1N2MessageSubscribePathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_N1N2MESSAGESUBSCRIBEPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_N1N2MESSAGESUBSCRIBEPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|imei-[0-9]{15}|imeisv-[0-9]{16}|.+)$").unwrap();
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
pub struct NonUeN2InfoUnSubscribePathParams {
	/// N2 info Subscription Identifier
	pub n2_notify_subscription_id: String,
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
pub struct ProvideDomainSelectionInfoPathParams {
	/// UE Context Identifier
	#[validate(
		regex(path = *RE_PROVIDEDOMAINSELECTIONINFOPATHPARAMS_UE_CONTEXT_ID),
	)]
	pub ue_context_id: String,
}

lazy_static::lazy_static! {
	static ref RE_PROVIDEDOMAINSELECTIONINFOPATHPARAMS_UE_CONTEXT_ID: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|.+)$").unwrap();
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
pub struct ProvideDomainSelectionInfoQueryParams {
	/// UE Context Information Class
	#[serde(rename = "info-class")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub info_class: Option<models::UeContextInfoClass>,
	/// Supported Features
	#[serde(rename = "supported-features")]
	#[validate(
		regex(path = *RE_PROVIDEDOMAINSELECTIONINFOQUERYPARAMS_SUPPORTED_FEATURES),
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_features: Option<String>,
	/// Old GUAMI
	#[serde(rename = "old-guami")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub old_guami: Option<crate::types::Object>,
}

lazy_static::lazy_static! {
	static ref RE_PROVIDEDOMAINSELECTIONINFOQUERYPARAMS_SUPPORTED_FEATURES: regex::Regex = regex::Regex::new(r"^[A-Fa-f0-9]*$").unwrap();
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
pub struct EnableUeReachabilityPathParams {
	/// UE Context Identifier
	pub ue_context_id: String,
}

///       requestBody:
//        content:
//          application/json-patch+json:
//            schema:
//              oneOf:
//                - type: array items: $ref: '#/components/schemas/AmfUpdateEventSubscriptionItem'
//                  minItems: 1
//                - type: array items: $ref: '#/components/schemas/AmfUpdateEventOptionItem'
//                  minItems: 1 maxItems: 1
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, smart_default::SmartDefault)]
pub enum ModifySubscriptionRequest {
	#[default]
	AmfUpdateEventSubscriptionItem(AmfUpdateEventSubscriptionItem),
	AmfUpdateEventOptionItem(AmfUpdateEventOptionItem),
}
