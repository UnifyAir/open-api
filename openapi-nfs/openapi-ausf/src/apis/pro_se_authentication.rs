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
pub enum CreateProSeEapSessionResponse {
	/// Successful authentication with CP-PRUK ID
	Status200
	(models::ProSeAuthenticationResult)
	= 200
	,
	/// ProSeAuthenticationCtx
	Status201
	{
		body: String,
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
	/// Bad request
	Status400
	(models::ProblemDetails)
	= 400
	,
	/// Forbidden
	Status403
	(models::ProblemDetails)
	= 403
	,
	/// Not Found
	Status404
	(models::ProblemDetails)
	= 404
	,
	/// Internal Server Error
	Status500
	(models::ProblemDetails)
	= 500,
}

impl DeserResponse for CreateProSeEapSessionResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProSeAuthenticationResult = serde_json::from_str(&data)?;

				Ok((status, CreateProSeEapSessionResponse::Status200
					(body)
				))
			}
			201 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: String = data;

				Ok((status, CreateProSeEapSessionResponse::Status201
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

				Ok((status, CreateProSeEapSessionResponse::Status307
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

				Ok((status, CreateProSeEapSessionResponse::Status308
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

				Ok((status, CreateProSeEapSessionResponse::Status400
					(body)
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateProSeEapSessionResponse::Status403
					(body)
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateProSeEapSessionResponse::Status404
					(body)
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateProSeEapSessionResponse::Status500
					(body)
				))
			}
			_ => Err(ReqError::UnexpectedResponseError(status.as_u16(), resp.text().await?, Backtrace::force_capture()))
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum DeleteProSeAuthenticationResultResponse {
	/// Expected response to a successful authentication result removal
	Status204
	= 204
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
	/// Bad request
	Status400
	(models::ProblemDetails)
	= 400
	,
	/// Not Found
	Status404
	(models::ProblemDetails)
	= 404
	,
	/// Internal Server Error
	Status500
	(models::ProblemDetails)
	= 500
	,
	/// Service Unavailable
	Status503
	(models::ProblemDetails)
	= 503
	,
	/// Generic Error
	Statusdefault
	= 0,
}

impl DeserResponse for DeleteProSeAuthenticationResultResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Status204
				))
			}
			307 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp.headers().get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID).map(|v| v.to_str().ok().map(|s| s.to_owned())).flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Status307
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

				Ok((status, DeleteProSeAuthenticationResultResponse::Status308
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

				Ok((status, DeleteProSeAuthenticationResultResponse::Status400
					(body)
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Status404
					(body)
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Status500
					(body)
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Status503
					(body)
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, DeleteProSeAuthenticationResultResponse::Statusdefault
				))
			}
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ProseAuthResponse {
	/// Use to handle or close the EAP session for 5G ProSe Remote UE
	Status200
	(models::ProSeEapSession)
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
	/// Bad request
	Status400
	(models::ProblemDetails)
	= 400
	,
	/// Internal Server Error
	Status500
	(models::ProblemDetails)
	= 500,
}

impl DeserResponse for ProseAuthResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProSeEapSession = serde_json::from_str(&data)?;

				Ok((status, ProseAuthResponse::Status200
					(body)
				))
			}
			307 => {
				let location: String = resp.headers().get(reqwest::header::LOCATION).ok_or(ReqError::RequiredHeaderNotFound("location".to_string(), Backtrace::force_capture()))?.to_str()?.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp.headers().get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID).map(|v| v.to_str().ok().map(|s| s.to_owned())).flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ProseAuthResponse::Status307
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

				Ok((status, ProseAuthResponse::Status308
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

				Ok((status, ProseAuthResponse::Status400
					(body)
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProseAuthResponse::Status500
					(body)
				))
			}
			_ => Err(ReqError::UnexpectedResponseError(status.as_u16(), resp.text().await?, Backtrace::force_capture()))
		}
	}
}


/// ProSeAuthentication
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ProSeAuthentication {
	/// CreateProSeEapSession - POST /nausf-auth/v1/prose-authentications
	async fn create_pro_se_eap_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::ProSeAuthenticationInfo,
	) -> Result<CreateProSeEapSessionResponse, String>;

	/// Deletes the authentication result in the UDM.
	///
	/// DeleteProSeAuthenticationResult - DELETE /nausf-auth/v1/prose-authentications/{authCtxId}/prose-auth
	async fn delete_pro_se_authentication_result(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeleteProSeAuthenticationResultPathParams,
	) -> Result<DeleteProSeAuthenticationResultResponse, String>;

	/// ProseAuth - POST /nausf-auth/v1/prose-authentications/{authCtxId}/prose-auth
	async fn prose_auth(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ProseAuthPathParams,
		body: Option<models::ProSeEapSession>,
	) -> Result<ProseAuthResponse, String>;
}
