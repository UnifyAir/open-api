#![allow(unused_qualifications)]

pub use oasbi::chf::types::{self as models, *};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostPathParams {
	/// a unique identifier for a charging data resource in a PLMN
	pub charging_data_ref: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostPathParams {
	/// a unique identifier for a charging data resource in a PLMN
	pub charging_data_ref: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeletePathParams {
	/// Identifies an individual spending limit retrieval subscription.
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutPathParams {
	/// Identifies an individual spending limit retrieval subscription.
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(untagged)]
pub enum NchfConvergedchargingV3ChargingdataPost400Response {
	ProblemDetails(models::ProblemDetails),
	ChargingDataResponse(models::ChargingDataResponse),
}
