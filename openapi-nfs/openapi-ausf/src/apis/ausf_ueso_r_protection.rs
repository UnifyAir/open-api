use crate::models;
use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use oasbi::{DeserResponse, ReqError};
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum CreateSoRProtectionResponse {
	/// SorSecurityInfo
	Status200
	(models::SorSecurityInfo)
	= 200
	,
	/// Temporary Redirect
	Status307
	{
		body: models::RedirectResponse,
		location:
			String
		,
		param_3gpp_sbi_target_nf_id:
			Option<
				String
			>,
    }
	= 307
	,
	/// Permanent Redirect
	Status308
	{
		body: models::RedirectResponse,
		location:
			String
		,
		param_3gpp_sbi_target_nf_id:
			Option<
				String
			>,
    }
	= 308
	,
	/// Service Unavailable
	Status503
	(models::ProblemDetails)
	= 503,
}

impl DeserResponse for CreateSoRProtectionResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SorSecurityInfo = serde_json::from_str(&data)?;

				Ok((status, CreateSoRProtectionResponse::Status200
					(body)
				))
			}
			307 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp.headers().get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID).map(|v| v.to_str().ok().map(|s| s.to_owned())).flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CreateSoRProtectionResponse::Status307
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
			}
			308 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp.headers().get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID).map(|v| v.to_str().ok().map(|s| s.to_owned())).flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CreateSoRProtectionResponse::Status308
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSoRProtectionResponse::Status503
					(body)
				))
			}
			_ => Err(ReqError::UnexpectedResponseError(status.as_u16(), resp.text().await?, Backtrace::force_capture()))
		}
	}
}


/// AusfUesoRProtection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AusfUesoRProtection {
	/// CreateSoRProtection - POST /nausf-sorprotection/v1/{supi}/ue-sor
	async fn create_so_r_protection(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::CreateSoRProtectionPathParams,
		body: models::SorInfo,
	) -> Result<CreateSoRProtectionResponse, String>;
}
