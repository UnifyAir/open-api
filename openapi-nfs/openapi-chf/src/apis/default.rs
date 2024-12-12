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
pub enum NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse {
	/// No Content.
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
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Not Found
	Status404(models::NchfConvergedchargingV3ChargingdataPost400Response) = 404,
	/// Gone
	Status410(models::common_models::ProblemDetails) = 410,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status204
				))
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

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status307
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
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

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status308
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status401
					(body)
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status404
					(body)
				))
			}
			410 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status410
					(body)
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status411
					(body)
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status413
					(body)
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status500
					(body)
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Status503
					(body)
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse::Statusdefault
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
pub enum NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse {
	/// OK. Updated Charging Data resource is returned
	Status200(models::ChargingDataResponse) = 200,
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
	Status400(models::NchfConvergedchargingV3ChargingdataPost400Response) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::NchfConvergedchargingV3ChargingdataPost400Response) = 403,
	/// Not Found
	Status404(models::NchfConvergedchargingV3ChargingdataPost400Response) = 404,
	/// Method Not Allowed
	Status405 = 405,
	/// Request Timeout
	Status408(models::common_models::ProblemDetails) = 408,
	/// Gone
	Status410(models::common_models::ProblemDetails) = 410,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ChargingDataResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status200(
						body,
					),
				))
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

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status307
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
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

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status308
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
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status400(
						body,
					),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status401(
						body,
					),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status403(
						body,
					),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status404(
						body,
					),
				))
			}
			405 => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status405,
				))
			}
			408 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status408(
						body,
					),
				))
			}
			410 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status410(
						body,
					),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status411(
						body,
					),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status413(
						body,
					),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status500(
						body,
					),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Status503(
						body,
					),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse::Statusdefault
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
pub enum NchfConvergedchargingV3ChargingdataPostResponse {
	/// Created
	Status201(models::ChargingDataResponse) = 201,
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
	Status400(models::NchfConvergedchargingV3ChargingdataPost400Response) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::NchfConvergedchargingV3ChargingdataPost400Response) = 403,
	/// Not Found
	Status404(models::NchfConvergedchargingV3ChargingdataPost400Response) = 404,
	/// Method Not Allowed
	Status405 = 405,
	/// Request Timeout
	Status408(models::common_models::ProblemDetails) = 408,
	/// Gone
	Status410(models::common_models::ProblemDetails) = 410,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfConvergedchargingV3ChargingdataPostResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			201 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ChargingDataResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status201(body),
				))
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

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status307 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
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

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status308 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::NchfConvergedchargingV3ChargingdataPost400Response =
					serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status404(body),
				))
			}
			405 => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status405,
				))
			}
			408 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status408(body),
				))
			}
			410 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status410(body),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status411(body),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status413(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status500(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfConvergedchargingV3ChargingdataPostResponse::Statusdefault,
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
pub enum NchfSpendinglimitcontrolV1SubscriptionsPostResponse {
	/// Success
	Status201 {
		body: models::SpendingLimitStatus,
		location: String,
	} = 201,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::common_models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::common_models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfSpendinglimitcontrolV1SubscriptionsPostResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			201 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SpendingLimitStatus = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status201 {
						body,
						location,
					},
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status404(body),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status411(body),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status413(body),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status415(body),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status500(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsPostResponse::Statusdefault,
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
pub enum NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse {
	/// No Content. Resource was succesfully deleted
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
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Too Many Requests
	Status429(models::common_models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status204,
				))
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

				Ok((status, NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status307
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
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

				Ok((status, NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status308
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
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status400(
						body,
					),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status401(
						body,
					),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status403(
						body,
					),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status404(
						body,
					),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status429(
						body,
					),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status500(
						body,
					),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Status503(
						body,
					),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse::Statusdefault
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
pub enum NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse {
	/// OK. Resource was succesfully modified and representation is returned
	Status200(models::SpendingLimitStatus) = 200,
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
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::common_models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::common_models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SpendingLimitStatus = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status200(
						body,
					),
				))
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

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status307 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
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

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status308 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status400(
						body,
					),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status401(
						body,
					),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status403(
						body,
					),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status404(
						body,
					),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status411(
						body,
					),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status413(
						body,
					),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status415(
						body,
					),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status429(
						body,
					),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status500(
						body,
					),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Status503(
						body,
					),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse::Statusdefault,
				))
			}
		}
	}
}

/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default {
	/// NchfConvergedchargingV3ChargingdataChargingDataRefReleasePost - POST
	/// /nchf-convergedcharging/v3/chargingdata/{ChargingDataRef}/release
	async fn nchf_convergedcharging_v3_chargingdata_charging_data_ref_release_post(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostPathParams,
		body: models::ChargingDataRequest,
	) -> Result<NchfConvergedchargingV3ChargingdataChargingDataRefReleasePostResponse, String>;

	/// NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePost - POST
	/// /nchf-convergedcharging/v3/chargingdata/{ChargingDataRef}/update
	async fn nchf_convergedcharging_v3_chargingdata_charging_data_ref_update_post(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostPathParams,
		body: models::ChargingDataRequest,
	) -> Result<NchfConvergedchargingV3ChargingdataChargingDataRefUpdatePostResponse, String>;

	/// NchfConvergedchargingV3ChargingdataPost - POST
	/// /nchf-convergedcharging/v3/chargingdata
	async fn nchf_convergedcharging_v3_chargingdata_post(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::ChargingDataRequest,
	) -> Result<NchfConvergedchargingV3ChargingdataPostResponse, String>;

	/// NchfSpendinglimitcontrolV1SubscriptionsPost - POST
	/// /nchf-spendinglimitcontrol/v1/subscriptions
	async fn nchf_spendinglimitcontrol_v1_subscriptions_post(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::SpendingLimitContext,
	) -> Result<NchfSpendinglimitcontrolV1SubscriptionsPostResponse, String>;

	/// NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDelete - DELETE
	/// /nchf-spendinglimitcontrol/v1/subscriptions/{subscriptionId}
	async fn nchf_spendinglimitcontrol_v1_subscriptions_subscription_id_delete(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeletePathParams,
	) -> Result<NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdDeleteResponse, String>;

	/// NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPut - PUT
	/// /nchf-spendinglimitcontrol/v1/subscriptions/{subscriptionId}
	async fn nchf_spendinglimitcontrol_v1_subscriptions_subscription_id_put(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutPathParams,
		body: models::SpendingLimitContext,
	) -> Result<NchfSpendinglimitcontrolV1SubscriptionsSubscriptionIdPutResponse, String>;
}
