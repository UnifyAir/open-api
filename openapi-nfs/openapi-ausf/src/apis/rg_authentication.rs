use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};
use oasbi::{DeserResponse, ReqError};
use crate::{models, types::*};
use std::backtrace::Backtrace;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum CreateRgAuthenticationResponse {
	/// RgAuthCtx
	Status201
	{
		body: models::RgAuthCtx,
		location:
			String,
	}
	= 201
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
	/// Bad Request from the AMF
	Status400
	(models::ProblemDetails)
	= 400
	,
	/// The UE is not allowed to be authenticated
	Status403
	(models::ProblemDetails)
	= 403
	,
	/// User does not exist in the HPLMN
	Status404
	(models::ProblemDetails)
	= 404,
}

impl DeserResponse for CreateRgAuthenticationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			201 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RgAuthCtx = serde_json::from_str(&data)?;

				Ok((status, CreateRgAuthenticationResponse::Status201
				{
					body,
					location,
				}
				))
			}
			307 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp.headers().get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID).map(|v| v.to_str().ok().map(|s| s.to_owned())).flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CreateRgAuthenticationResponse::Status307
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

				Ok((status, CreateRgAuthenticationResponse::Status308
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateRgAuthenticationResponse::Status400
					(body)
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateRgAuthenticationResponse::Status403
					(body)
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateRgAuthenticationResponse::Status404
					(body)
				))
			}
			_ => Err(ReqError::UnexpectedResponseError(status.as_u16(), resp.text().await?, Backtrace::force_capture()))
		}
	}
}


/// RgAuthentication
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait RgAuthentication {
	/// CreateRgAuthentication - POST /nausf-auth/v1/rg-authentications
	async fn create_rg_authentication(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::RgAuthenticationInfo,
	) -> Result<CreateRgAuthenticationResponse, String>;
}
