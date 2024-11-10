#![allow(unused_qualifications)]

pub use oasbi::pcf::types::{self as models, *};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAmEventsSubscPathParams {
	/// String identifying the Individual Application AM Context resource.
	pub app_am_context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAmEventsSubscPathParams {
	/// String identifying the AM Policy Events Subscription subresource.
	pub app_am_context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteEventsSubscPathParams {
	/// String identifying the Individual Application Session Context resource.
	pub app_session_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateEventsSubscPathParams {
	/// String identifying the Events Subscription resource.
	pub app_session_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteIndividualAmPolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReadIndividualAmPolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReportObservedEventTriggersForIndividualAmPolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAppAmContextPathParams {
	/// String identifying the Individual Application AM Context resource.
	pub app_am_context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAppAmContextPathParams {
	/// String identifying the resource.
	pub app_am_context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModAppAmContextPathParams {
	/// String identifying the resource.
	pub app_am_context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAppSessionPathParams {
	/// String identifying the Individual Application Session Context resource.
	pub app_session_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAppSessionPathParams {
	/// String identifying the resource.
	pub app_session_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModAppSessionPathParams {
	/// String identifying the resource.
	pub app_session_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetBdtPolicyPathParams {
	/// String identifying the individual BDT policy resource in the PCF.
	pub bdt_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateBdtPolicyPathParams {
	/// String identifying the individual BDT policy resource in the PCF.
	pub bdt_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteMbsAppSessionCtxtPathParams {
	/// Contains the identifier of the Individual MBS Application Session
	/// Context resource.
	pub context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMbsAppSessionCtxtPathParams {
	/// Contains the identifier of the Individual MBS Application Session
	/// Context resource.
	pub context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModifyMbsAppSessionCtxtPathParams {
	/// Contains the identifier of the Individual MBS Application Session
	/// Context resource.
	pub context_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteIndMbsPolicyPathParams {
	/// Contains the identifier of the concerned Individual MBS Policy resource.
	pub mbs_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetIndMbsPolicyPathParams {
	/// Contains the identifier of the concerned Individual MBS Policy resource.
	pub mbs_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateIndMbsPolicyPathParams {
	/// Contains the identifier of the concerned Individual MBS Policy resource.
	pub mbs_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeletePcEventExposureSubscPathParams {
	/// Policy Control Event Subscription ID.
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPcEventExposureSubscPathParams {
	/// Policy Control Event Subscription ID.
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PutPcEventExposureSubscPathParams {
	/// Policy Control Event Subscription ID.
	pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteSmPolicyPathParams {
	/// Identifier of a policy association
	pub sm_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSmPolicyPathParams {
	/// Identifier of a policy association
	pub sm_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateSmPolicyPathParams {
	/// Identifier of a policy association
	pub sm_policy_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteIndividualUePolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReadIndividualUePolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReportObservedEventTriggersForIndividualUePolicyAssociationPathParams {
	/// Identifier of a policy association
	pub pol_asso_id: String,
}
