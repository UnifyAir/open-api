#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{types::*};
use oasbi::{DeserResponse, ReqError};
use serde_with::skip_serializing_none;

pub use oasbi::ausf::types::*;
pub use oasbi::ausf::types::{*, self as models};

#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateSoRProtectionPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CREATESORPROTECTIONPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
        static ref RE_CREATESORPROTECTIONPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
    }




#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteProSeAuthenticationResultPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProseAuthPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Create5gAkaAuthenticationResultPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Delete5gAkaAuthenticationResultPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteEapAuthenticationResultPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EapAuthMethodPathParams {
	pub auth_ctx_id: String,
}


#[derive(
	Debug,
	Clone,
	serde::Serialize,
	serde::Deserialize,
	validator::Validate,
	smart_default::SmartDefault
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateUpuProtectionPathParams {
	/// Identifier of the UE
	#[validate(
		regex(path = *RE_CREATEUPUPROTECTIONPATHPARAMS_SUPI),
	)]
	pub supi: String,
}

lazy_static::lazy_static! {
        static ref RE_CREATEUPUPROTECTIONPATHPARAMS_SUPI: regex::Regex = regex::Regex::new(r"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$").unwrap();
    }


