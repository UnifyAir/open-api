use std::backtrace::Backtrace;

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use oasbi::{DeserResponse, ReqError};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum CancelLocationResponse {
	/// Expected response to a successful cancellation
	Status204 = 204,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 308,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Gateway Timeout
	Status504(models::ProblemDetails) = 504,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for CancelLocationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, CancelLocationResponse::Status204))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status307 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status503(body)))
			}
			504 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelLocationResponse::Status504(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, CancelLocationResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ProvideLocationInfoResponse {
	/// Expected response to a valid request
	Status200(models::ProvideLocInfo) = 200,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 308,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for ProvideLocationInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProvideLocInfo = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status200(body)))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status307 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvideLocationInfoResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ProvideLocationInfoResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ProvidePositioningInfoResponse {
	/// Expected response to a valid request
	Status200(models::ProvidePosInfo) = 200,
	/// Successful accept of location request with no information returned.
	Status204 = 204,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 308,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Gateway Timeout
	Status504(models::ProblemDetails) = 504,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for ProvidePositioningInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProvidePosInfo = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, ProvidePositioningInfoResponse::Status204))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status307 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status403(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status503(body)))
			}
			504 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ProvidePositioningInfoResponse::Status504(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ProvidePositioningInfoResponse::Statusdefault))
			}
		}
	}
}

/// IndividualUeContextDocumentLocation
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualUeContextDocumentLocation {
	/// Namf_Location CancelLocation service operation.
	///
	/// CancelLocation - POST /namf-loc/v1/{ueContextId}/cancel-pos-info
	async fn cancel_location(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::CancelLocationPathParams,
		body: models::CancelPosInfo,
	) -> Result<CancelLocationResponse, String>;

	/// Namf_Location ProvideLocationInfo service Operation.
	///
	/// ProvideLocationInfo - POST /namf-loc/v1/{ueContextId}/provide-loc-info
	async fn provide_location_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ProvideLocationInfoPathParams,
		body: models::RequestLocInfo,
	) -> Result<ProvideLocationInfoResponse, String>;

	/// Namf_Location ProvidePositioningInfo service Operation.
	///
	/// ProvidePositioningInfo - POST
	/// /namf-loc/v1/{ueContextId}/provide-pos-info
	async fn provide_positioning_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ProvidePositioningInfoPathParams,
		body: models::RequestPosInfo,
	) -> Result<ProvidePositioningInfoResponse, String>;
}
